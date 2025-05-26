use wicked_waifus_data::RawVectorData;
use wicked_waifus_protocol::Vector;
use wicked_waifus_data::pb_components::teleport::TeleportPosition;
use wicked_waifus_protocol_internal::VectorData;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn to_protobuf(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn from_save(data: VectorData) -> Self {
        Self {
            x: data.x,
            y: data.y,
            z: data.z,
        }
    }

    pub fn save_data(&self) -> VectorData {
        VectorData {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn from_data(data: &wicked_waifus_data::VectorData) -> Self {
        Self {
            x: data.get_x(),
            y: data.get_y(),
            z: data.get_z(),
        }
    }

    pub fn from_raw_scaled(transform: &RawVectorData, scale: &RawVectorData) -> Self {
        Self {
            x: transform.x / scale.x,
            y: transform.y / scale.y,
            z: transform.z / scale.z,
        }
    }

    pub fn add_teleport_position(&mut self, teleport_position: &TeleportPosition) {
        self.x += teleport_position.x.unwrap_or_default();
        self.y += teleport_position.y.unwrap_or_default();
        self.z += teleport_position.z.unwrap_or_default();
    }
}