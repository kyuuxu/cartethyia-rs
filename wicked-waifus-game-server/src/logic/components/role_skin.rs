use crate::logic::ecs::component::Component;

pub struct RoleSkin {
    pub skin_id: i32,
}

impl Component for RoleSkin {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.role_skin_id = self.skin_id
    }
}
