use wicked_waifus_protocol_internal::PlayerMonthCardData;

pub struct PlayerMonthCard {
    pub days: i32,
    pub last_received_day: i32
}

impl PlayerMonthCard {
    pub fn load_from_save(data: PlayerMonthCardData) -> Self {
        Self {
            days: data.days,
            last_received_day: data.last_received_day,
        }
    }

    pub fn build_save_data(&self) -> PlayerMonthCardData {
        PlayerMonthCardData {
            days: self.days,
            last_received_day: self.last_received_day,
        }
    }
}

impl Default for PlayerMonthCard {
    fn default() -> Self {
        Self {
            days: 3650,
            last_received_day: wicked_waifus_commons::time_util::unix_days() - 1,
        }
    }
}
