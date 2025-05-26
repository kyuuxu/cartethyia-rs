use wicked_waifus_protocol::{EntityComponentPb, TagComponentPb};
use wicked_waifus_protocol::entity_component_pb::ComponentPb;

use crate::logic::ecs::component::Component;

pub struct Tag {
    // TODO
    pub gameplay_tags: Vec<i32>,
    pub entity_common_tags: Vec<i32>,
    pub init_gameplay_tag: bool,
}

impl Component for Tag {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::TagComponent(TagComponentPb {
                gameplay_tags: vec![],
                entity_common_tags: self.entity_common_tags.clone(),
                init_gameplay_tag: self.init_gameplay_tag,
            })),
        })
    }
}
