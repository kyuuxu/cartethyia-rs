use wicked_waifus_protocol::{ConcomitantsComponentPb, EntityComponentPb};
use wicked_waifus_protocol::entity_component_pb::ComponentPb;

use crate::logic::ecs::component::Component;

pub struct Concomitant {
    pub vision_entity_id: i64,
    pub custom_entity_ids: Vec<i64>,
    pub phantom_role_id: i64,
}

impl Component for Concomitant {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::ConcomitantsComponentPb(ConcomitantsComponentPb {
                vision_entity_id: self.vision_entity_id,
                custom_entity_ids: self.custom_entity_ids.clone(),
                phantom_role_id: self.phantom_role_id,
            })),
        })
    }
}