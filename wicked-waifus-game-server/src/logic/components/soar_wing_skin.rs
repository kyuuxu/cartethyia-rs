use crate::logic::ecs::component::Component;

pub struct SoarWingSkin {
    pub skin_id: i32,
}

impl Component for SoarWingSkin {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.soar_wing_skin_id = self.skin_id
    }
}
