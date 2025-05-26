use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicI32, Ordering};
use wicked_waifus_protocol::FightBuffInformation;

pub struct BufManager {
    active_buf_set: HashMap<i32, FightBuffInformation>,
    next_handle: AtomicI32,
    recycled_handles: HashMap<i32, VecDeque<i32>>,
}

impl BufManager {
    const PERMANENT_ROLE_BUFFS: &'static [i64] = &[
        3003,      // Remove wall run prohibition
        3004,      // Remove gliding prohibition
        1213,      // Reduce stamina while flying
        1214,      // Reduce stamina while flying in sprint
        1215,      // Reduce stamina while flying up in sprint
        1216,      // Reduce stamina while flying down in sprint
        640012051, // Allow flying -> tag: 1151923109
    ];

    pub fn create(&mut self, buf: &mut FightBuffInformation) {
        let handle = self
            .recycled_handles
            .get_mut(&buf.handle_id)
            .and_then(|ids| ids.pop_front())
            .unwrap_or_else(|| self.next_handle.fetch_add(1, Ordering::Relaxed));

        buf.handle_id = handle;
        buf.server_id = handle;
        buf.message_id = handle as i64;

        self.active_buf_set.entry(handle).or_insert(buf.clone());
    }

    #[inline(always)]
    pub fn remove_entity_buffs(&mut self, entity_id: i64) {
        let handles = self.active_buf_set.iter()
            .filter(|(_, buff)| buff.entity_id == entity_id)
            .map(|(&handle, _)| handle)
            .collect::<Vec<_>>();
        for handle in handles {
            self.remove(handle);
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, handle: i32) -> bool {
        if let Some(buf) = self.active_buf_set.remove(&handle) {
            self.recycled_handles
                .entry(handle)
                .or_default()
                .push_back(buf.handle_id);
            true
        } else {
            false
        }
    }

    pub fn create_permanent_buffs(&mut self, origin_id: i64) -> Vec<FightBuffInformation> {
        Self::PERMANENT_ROLE_BUFFS
            .iter()
            .map(|&id| {
                let mut buff = FightBuffInformation {
                    handle_id: 0,
                    buff_id: id,
                    level: 1,
                    stack_count: 1,
                    instigator_id: origin_id,
                    entity_id: origin_id,
                    apply_type: 0,
                    duration: -1f32,
                    left_duration: -1f32,
                    context: vec![],
                    is_active: true,
                    server_id: 0,
                    message_id: 0,
                };
                self.create(&mut buff);
                buff
            })
            .collect::<Vec<_>>()
    }
}

impl Default for BufManager {
    fn default() -> Self {
        Self {
            active_buf_set: Default::default(),
            next_handle: AtomicI32::new(1),
            recycled_handles: Default::default(),
        }
    }
}
