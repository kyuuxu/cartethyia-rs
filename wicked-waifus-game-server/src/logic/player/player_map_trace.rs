use std::collections::HashSet;
use wicked_waifus_protocol_internal::PlayerMapTraceData;

pub struct PlayerMapTrace {
    pub traces: HashSet<i32>,
}

impl PlayerMapTrace {
    pub fn load_from_save(data: PlayerMapTraceData) -> Self {
        Self {
            traces: data.traces.iter().cloned().collect(),
        }
    }

    pub fn build_save_data(&self) -> PlayerMapTraceData {
        PlayerMapTraceData {
            traces: self.traces.iter().cloned().collect(),
        }
    }
}

impl Default for PlayerMapTrace {
    fn default() -> Self {
        Self {
            traces: HashSet::new(),
        }
    }
}
