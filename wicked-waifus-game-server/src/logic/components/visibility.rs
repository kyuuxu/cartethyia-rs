use crate::logic::ecs::component::Component;

pub struct Visibility {
    pub is_visible: bool,
    pub is_actor_visible: bool,
}

impl Component for Visibility {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.is_visible = self.is_visible;
        pb.is_actor_visible = self.is_actor_visible;
    }
}
