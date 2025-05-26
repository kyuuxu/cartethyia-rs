use wicked_waifus_protocol::{EntityComponentPb, StateTagComponentPb};
use wicked_waifus_protocol::entity_component_pb::ComponentPb;

use crate::logic::ecs::component::Component;

pub struct StateTag {
    pub state_tag_id: i32
}

impl Component for StateTag {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::StateTagComponentPb(StateTagComponentPb {
                state_tag_id: 0,
            })),
        })
    }
}
