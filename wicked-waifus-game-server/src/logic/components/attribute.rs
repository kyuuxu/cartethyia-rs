use wicked_waifus_data::BasePropertyData;
use wicked_waifus_protocol::{
    entity_component_pb::ComponentPb, AttrData, AttributeComponentPb, EAttributeType,
    EntityComponentPb, LivingStatus,
};
use std::collections::HashMap;

use crate::logic::ecs::component::Component;
use crate::logic::utils::load_role_info::attribute_from_data;

pub struct Attribute {
    pub attr_map: HashMap<EAttributeType, (i32, i32)>,
    pub hardness_mode_id: i32,
    pub rage_mode_id: i32,
}

impl Component for Attribute {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.living_status = (if self.is_alive() {
            LivingStatus::Alive
        } else {
            LivingStatus::Dead
        })
        .into();

        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::AttributeComponent(
                self.build_entity_attribute(),
            )),
        });
    }
}

impl Attribute {
    pub fn is_alive(&self) -> bool {
        self.attr_map
            .get(&EAttributeType::Life)
            .copied()
            .unwrap_or_default()
            .0
            > 0
    }

    #[inline(always)]
    pub fn from_data(base_property: &BasePropertyData,
                     hardness_mode_id: Option<i32>,
                     rage_mode_id: Option<i32>) -> Self {
        Self {
            attr_map: attribute_from_data(base_property),
            hardness_mode_id: hardness_mode_id.unwrap_or_default(),
            rage_mode_id: rage_mode_id.unwrap_or_default(),
        }
    }

    #[inline(always)]
    pub fn build_entity_attribute(&self) -> AttributeComponentPb {
        AttributeComponentPb {
            attr_data: self
                .attr_map
                .iter()
                .map(|(ty, (base, incr))| AttrData {
                    attribute_type: (*ty).into(),
                    current_value: *base,
                    value_increment: *incr,
                })
                .collect(),
            hardness_mode_id: self.hardness_mode_id,
            rage_mode_id: self.rage_mode_id,
        }
    }
}
