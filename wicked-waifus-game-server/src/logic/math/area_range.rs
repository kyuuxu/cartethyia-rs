use crate::logic::math::Vector3f;

pub struct Area {
    pub center: Vector3f,
    pub radius: f32,
}

pub struct SquareArea(pub Area);
pub struct CircleArea(pub Area);
pub struct CubeArea(pub Area);
pub struct SphereArea(pub Area);

pub trait Ranger {
    fn in_range(&self, entity: Vector3f) -> bool;
}

impl Ranger for SquareArea {
    /// Computation of range in a square is done via 2D simple bound checks.
    fn in_range(&self, pos: Vector3f) -> bool {
        (self.0.center.x - self.0.radius <= pos.x && pos.x <= self.0.center.x + self.0.radius) &&
            (self.0.center.y - self.0.radius <= pos.y && pos.y <= self.0.center.y + self.0.radius)
    }
}

impl Ranger for CircleArea {
    /// Computation of range in a sphere is done via 2D Euclidean distance formula.
    fn in_range(&self, pos: Vector3f) -> bool {
        // TODO: Check for overflows in squaring??
        let distance_squared = (pos.x - self.0.center.x).powi(2) +
            (pos.y - self.0.center.y).powi(2);
        let radius_squared = self.0.radius * self.0.radius;
        distance_squared <= radius_squared
    }
}

impl Ranger for CubeArea {
    /// Computation of range in a cube is done via 3D simple bound checks.
    fn in_range(&self, pos: Vector3f) -> bool {
        (self.0.center.x - self.0.radius <= pos.x && pos.x <= self.0.center.x + self.0.radius) &&
            (self.0.center.y - self.0.radius <= pos.y && pos.y <= self.0.center.y + self.0.radius) &&
            (self.0.center.z - self.0.radius <= pos.z && pos.z <= self.0.center.z + self.0.radius)
    }
}

impl Ranger for SphereArea {
    /// Computation of range in a sphere is done via 3D Euclidean distance formula.
    fn in_range(&self, pos: Vector3f) -> bool {
        // TODO: Check for overflows in squaring??
        let distance_squared = (pos.x - self.0.center.x).powi(2) +
            (pos.y - self.0.center.y).powi(2) +
            (pos.z - self.0.center.z).powi(2);
        let radius_squared = self.0.radius * self.0.radius;
        distance_squared <= radius_squared
    }
}