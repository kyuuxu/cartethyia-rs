use wicked_waifus_protocol::entity_component_pb::ComponentPb;
use wicked_waifus_protocol::{EntityComponentPb, WeaponSkinComponentPb};
use crate::logic::ecs::component::Component;

pub struct WeaponSkin {
    pub skin_id: i32,
}

impl Component for WeaponSkin {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::WeaponSkinComponentPb(WeaponSkinComponentPb {
                weapon_skin_id: self.skin_id,
            })),
        })
    }
}
