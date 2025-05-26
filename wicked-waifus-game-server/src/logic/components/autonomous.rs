use wicked_waifus_protocol::{AutonomousComponentPb, EntityComponentPb};
use wicked_waifus_protocol::entity_component_pb::ComponentPb;

use crate::logic::ecs::component::Component;

pub struct Autonomous {
    pub autonomous_id: i32
}

impl Component for Autonomous {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::AutonomousComponentPb(AutonomousComponentPb {
                autonomous_id: self.autonomous_id,
                ji: vec![],
            })),
        })
    }
}
