use wicked_waifus_protocol::{EntityComponentPb, summon::SummonerComponentPb};
use wicked_waifus_protocol::entity_component_pb::ComponentPb;

use crate::logic::ecs::component::Component;

pub struct Summoner {
    pub summon_cfg_id: i32,
    pub summon_skill_id: i32,
    pub summon_type: i32,
}

impl Component for Summoner {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::SummonerComponent(SummonerComponentPb {
                summoner_id: pb.id + 1,
                summon_cfg_id: self.summon_cfg_id,
                summon_skill_id: self.summon_skill_id,
                player_id: pb.player_id,
                r#type: self.summon_type,
            })),
        })
    }
}