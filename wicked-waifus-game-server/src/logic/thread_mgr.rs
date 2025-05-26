use wicked_waifus_commons::time_util;
use wicked_waifus_protocol_internal::PlayerSaveData;
use wicked_waifus_protocol::{message::Message, AfterJoinSceneNotify, EnterGameResponse, JoinSceneNotify, SilenceNpcNotify, TransitionOptionPb};
use std::collections::hash_map::Entry::Vacant;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc, OnceLock,
    },
    thread,
    time::Duration,
};
use super::{ecs::world::World, player::Player, utils::world_util};
use crate::logic::ecs::world::WorldEntity;
use crate::{logic, player_save_task::{self, PlayerSaveReason}, session::Session};

pub enum LogicInput {
    AddPlayer {
        player_id: i32,
        enter_rpc_id: u16,
        session: Arc<Session>,
        player_save_data: PlayerSaveData,
    },
    RemovePlayer {
        player_id: i32,
    },
    ProcessMessage {
        player_id: i32,
        message: Message,
    },
}

#[derive(Clone)]
pub struct LogicThreadHandle {
    sender: mpsc::Sender<LogicInput>,
    load: Arc<AtomicUsize>,
}

static THREAD_HANDLES: OnceLock<Box<[LogicThreadHandle]>> = OnceLock::new();

pub fn start_logic_threads(num_threads: usize) {
    if THREAD_HANDLES.get().is_some() {
        tracing::error!("start_logic_threads: logic threads are already running!");
        return;
    }

    let _ = THREAD_HANDLES.set(
        (0..num_threads)
            .map(|_| {
                let (tx, rx) = mpsc::channel();
                let load = Arc::new(AtomicUsize::new(0));

                let handle = LogicThreadHandle {
                    sender: tx,
                    load: load.clone(),
                };

                thread::spawn(move || logic_thread_func(rx, load));
                handle
            })
            .collect(),
    );
}

// Thread-local logic state
struct LogicState {
    thread_load: Arc<AtomicUsize>, // shared parameter for load-balancing
    worlds: HashMap<i32, Rc<RefCell<World>>>, // owner_id - world
    players: HashMap<i32, RefCell<Player>>, // id - player
}

fn logic_thread_func(receiver: mpsc::Receiver<LogicInput>, load: Arc<AtomicUsize>) {
    const RECV_TIMEOUT: Duration = Duration::from_millis(50);
    const PLAYER_SAVE_PERIOD: u64 = 30;

    let mut state = LogicState {
        thread_load: load,
        worlds: HashMap::new(),
        players: HashMap::new(),
    };

    let mut input_queue = VecDeque::with_capacity(32);

    loop {
        if let Ok(input) = receiver.recv_timeout(RECV_TIMEOUT) {
            input_queue.push_back(input);

            while let Ok(input) = receiver.try_recv() {
                input_queue.push_back(input);
            }
        }

        while let Some(input) = input_queue.pop_front() {
            handle_logic_input(&mut state, input);
        }

        state.worlds.values().for_each(|world| {
            let mut world = world.borrow_mut();
            let mut players = world
                .player_ids()
                .flat_map(|id| state.players.get(id).map(|pl| pl.borrow_mut()))
                .collect::<Box<_>>();

            super::systems::tick_systems(&mut world, &mut players);
        });

        state.players.values().for_each(|player| {
            let mut player = player.borrow_mut();
            if time_util::unix_timestamp() - player.last_save_time > PLAYER_SAVE_PERIOD {
                player_save_task::push(
                    player.basic_info.id,
                    player.build_save_data(),
                    PlayerSaveReason::PeriodicalSave,
                );

                player.last_save_time = time_util::unix_timestamp();
            }
        })
    }
}

fn handle_logic_input(state: &mut LogicState, input: LogicInput) {
    match input {
        LogicInput::AddPlayer {
            player_id,
            enter_rpc_id,
            session,
            player_save_data,
        } => {
            let (player, is_player) = if let Vacant(e) = state.players.entry(player_id) {
                (
                    e.insert(RefCell::new(Player::load_from_save(player_save_data))),
                    true,
                )
            } else {
                if let Some(player) = state.players.get_mut(&player_id) {
                    (player, false)
                } else {
                    tracing::warn!("logic_thread: get player requested, but player {player_id} with data doesn't exist");
                    return;
                }
            };

            let mut player = player.borrow_mut();
            if is_player {
                player
                    .world
                    .borrow_mut()
                    .world_entitys
                    .insert(player.basic_info.cur_map_id, WorldEntity::default());
                state.worlds.insert(player_id, player.world.clone());
            }

            player.init();
            player.set_session(session);
            player.respond(EnterGameResponse::default(), enter_rpc_id);
            player.notify_general_data();

            player
                .world
                .borrow_mut()
                .set_in_world_player_data(player.build_in_world_player());

            world_util::add_player_entities(&player);
            let scene_info = world_util::build_scene_information(&player);

            player.notify(SilenceNpcNotify::default());

            player.notify(JoinSceneNotify {
                scene_info: Some(scene_info),
                max_entity_id: i64::MAX,
                transition_option: Some(TransitionOptionPb::default()),
            });

            player.notify(AfterJoinSceneNotify::default());
            player.notify(player.build_update_formation_notify());

            let map = logic::utils::quadrant_util::get_map(player.basic_info.cur_map_id);
            let quadrant_id = map.get_quadrant_id(
                player.location.position.position.x * 100.0,
                player.location.position.position.y * 100.0,
            );
            player.quadrant_id = quadrant_id;
            player.notify_month_card();

            let entities = map.get_initial_entities(quadrant_id);
            world_util::add_entities(&player, &entities, false);

            drop(player);

            state
                .thread_load
                .store(state.players.len(), Ordering::Relaxed);
        }
        LogicInput::ProcessMessage { player_id, message } => {
            let Some(player) = state.players.get_mut(&player_id) else {
                tracing::warn!("logic_thread: process message requested, but player with id {player_id} doesn't exist");
                return;
            };

            super::handler::handle_logic_message(&mut player.borrow_mut(), message);
        }
        LogicInput::RemovePlayer { player_id } => {
            let Some(player) = state.players.remove(&player_id) else {
                tracing::warn!(
                    "logic_thread: player remove requested, but it doesn't exist (id: {player_id})"
                );
                return;
            };

            let _ = state.worlds.remove(&player_id);
            // TODO: kick co-op players from removed world
            // TODO: Remove all entities

            player_save_task::push(
                player_id,
                player.borrow().build_save_data(),
                PlayerSaveReason::PlayerLogicStopped,
            );
        }
    }
}

impl LogicThreadHandle {
    pub fn input(&self, input: LogicInput) {
        let _ = self.sender.send(input);
    }
}

pub fn get_least_loaded_thread() -> LogicThreadHandle {
    let handles = THREAD_HANDLES.get().unwrap();
    handles
        .iter()
        .min_by_key(|h| h.load.load(Ordering::Relaxed))
        .unwrap()
        .clone()
}
