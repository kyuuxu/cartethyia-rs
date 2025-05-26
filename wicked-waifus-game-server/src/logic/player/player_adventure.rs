use wicked_waifus_protocol_internal::{PlayerAdventureGlobalStatusData, PlayerAdventureStatusData, PlayerAdventureTaskStatusData};

use crate::config;

pub struct PlayerAdventureTaskStatus {
    pub id: i32,
    pub state: i32,
    pub progress: i32,
}

pub struct PlayerAdventureGlobalStatus {
    pub adventure_task_status: Vec<PlayerAdventureTaskStatus>,
    pub now_chapter: i32,
    pub received_chapter: i32,
}

pub struct PlayerAdventureStatus {
    pub status: Vec<PlayerAdventureGlobalStatus>,
}

impl PlayerAdventureStatus {
    pub fn load_from_save(data: PlayerAdventureStatusData) -> Self {
        Self {
            status: data.status.iter()
                .map(|global_status| {
                    PlayerAdventureGlobalStatus {
                        adventure_task_status: global_status.status.iter()
                            .map(|status| {
                                PlayerAdventureTaskStatus {
                                    id: status.id,
                                    state: status.state,
                                    progress: status.progress,
                                }
                            })
                            .collect::<Vec<_>>(),
                        now_chapter: global_status.now_chapter,
                        received_chapter: global_status.received_chapter,
                    }
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn build_save_data(&self) -> PlayerAdventureStatusData {
        PlayerAdventureStatusData {
            status: self.status.iter()
                .map(|global_status| {
                    PlayerAdventureGlobalStatusData {
                        status: global_status.adventure_task_status.iter()
                            .map(|status| {
                                PlayerAdventureTaskStatusData {
                                    id: status.id,
                                    state: status.state,
                                    progress: status.progress,
                                }
                            })
                            .collect::<Vec<_>>(),
                        now_chapter: global_status.now_chapter,
                        received_chapter: global_status.received_chapter,
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl Default for PlayerAdventureStatus {
    fn default() -> Self {
        if config::get_config().default_unlocks.unlock_all_adventures {
            let mut max_chapter = 1;
            for task_data in wicked_waifus_data::adventure_task_data::iter() {
                if task_data.chapter_id > max_chapter {
                    max_chapter = task_data.chapter_id;
                }
            }
            Self {
                status: vec![
                    PlayerAdventureGlobalStatus {
                        adventure_task_status: wicked_waifus_data::adventure_task_data::iter()
                            .map(|task| PlayerAdventureTaskStatus {
                                id: task.id,
                                state: 2,
                                progress: task.need_progress,
                            })
                            .collect::<Vec<_>>(),
                        now_chapter: max_chapter,
                        received_chapter: max_chapter,
                    }
                ],
            }
        } else {
            Self {
                status: vec![
                    PlayerAdventureGlobalStatus {
                        adventure_task_status: wicked_waifus_data::adventure_task_data::iter()
                            .map(|task| PlayerAdventureTaskStatus {
                                id: task.id,
                                state: 0,
                                progress: 0,
                            })
                            .collect::<Vec<_>>(),
                        now_chapter: 1,
                        received_chapter: 0,
                    }
                ],
            }
        }
    }
}