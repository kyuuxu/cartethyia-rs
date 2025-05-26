use crate::logic::ecs::component::Component;

pub struct ParaglidingSkin {
    pub skin_id: i32,
}

impl Component for ParaglidingSkin {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.paragliding_skin_id = self.skin_id
    }
}
