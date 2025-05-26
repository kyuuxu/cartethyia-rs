use wicked_waifus_protocol_internal::PlayerAdviceData;

pub struct PlayerAdviceConfig {
    pub is_show: bool,
}

impl PlayerAdviceConfig {
    pub fn load_from_save(data: PlayerAdviceData) -> Self {
        PlayerAdviceConfig {
            is_show: data.is_show,
        }
    }

    pub fn build_save_data(&self) -> PlayerAdviceData {
        PlayerAdviceData {
            is_show: self.is_show,
        }
    }
}

impl Default for PlayerAdviceConfig {
    fn default() -> Self {
        Self {
            is_show: true,
        }
    }
}