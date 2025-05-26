use wicked_waifus_protocol_internal::{
    PlayerTutorialData, PlayerTutorialsData,
};

use crate::config;

#[derive(Clone)]
pub struct PlayerTutorial {
    pub id: i32,
    pub create_time: u32,
    pub get_award: bool,
}

pub struct PlayerTutorials {
    pub tutorials: Vec<PlayerTutorial>,
}

impl PlayerTutorials {
    pub fn load_from_save(data: PlayerTutorialsData) -> Self {
        Self {
            tutorials: data
                .tutorials
                .iter()
                .map(|tutorial| PlayerTutorial {
                    id: tutorial.id,
                    create_time: tutorial.create_time,
                    get_award: tutorial.get_award,
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn build_save_data(&self) -> PlayerTutorialsData {
        PlayerTutorialsData {
            tutorials: self
                .tutorials
                .iter()
                .map(|tutorial| PlayerTutorialData {
                    id: tutorial.id,
                    create_time: tutorial.create_time,
                    get_award: tutorial.get_award,
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl Default for PlayerTutorials {
    fn default() -> Self {
        if config::get_config().default_unlocks.unlock_all_tutorials {
            Self {
                tutorials: wicked_waifus_data::guide_tutorial_data::iter()
                    .map(|tutorial| PlayerTutorial {
                        id: tutorial.id,
                        create_time: wicked_waifus_commons::time_util::unix_timestamp() as u32,
                        get_award: false,
                    })
                    .collect::<Vec<_>>(),
            }
        } else {
            Self { tutorials: vec![] }
        }
    }
}
