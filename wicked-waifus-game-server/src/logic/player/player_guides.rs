use std::collections::HashSet;

use wicked_waifus_protocol_internal::PlayerGuidesData;

use crate::config;

pub struct PlayerGuides {
    pub started_guides: HashSet<i32>,
    pub finished_guides: HashSet<i32>,
}

impl PlayerGuides {
    pub fn load_from_save(data: PlayerGuidesData) -> Self {
        PlayerGuides {
            started_guides: data.started_guides.iter().cloned().collect(),
            finished_guides: data.finished_guides.iter().cloned().collect(),
        }
    }

    pub fn build_save_data(&self) -> PlayerGuidesData {
        PlayerGuidesData {
            started_guides: self.started_guides.iter().cloned().collect(),
            finished_guides: self.finished_guides.iter().cloned().collect(),
        }
    }
}

impl Default for PlayerGuides {
    fn default() -> Self {
        let finished_guides = if config::get_config().default_unlocks.unlock_all_guides {
            let guide_group_data = wicked_waifus_data::guide_group_data::iter();
            guide_group_data.map(|group| group.id).collect()
        } else {
            HashSet::new()
        };
        Self {
            started_guides: HashSet::new(),
            finished_guides,
        }
    }
}