use wicked_waifus_protocol::{EntityComponentPb, InteractComponentPb};
use wicked_waifus_protocol::entity_component_pb::ComponentPb;

use crate::logic::ecs::component::Component;

pub struct Interact {
    // TODO
}

impl Component for Interact {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::InteractComponent(InteractComponentPb {
                dynamic_interact_infos: vec![],
                random_interact_index: vec![],
                interacting: false,
            })),
        })
    }
}
