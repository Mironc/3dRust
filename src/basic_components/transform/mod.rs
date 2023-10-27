use glam::*;
use specs::prelude::*;
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub right: Vec3,
}
impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: vec3(1., 1., 1.),
            forward: Vec3::Z,
            up: Vec3::Y,
            right: Vec3::X,
        }
    }
    pub fn with_position(translate: Vec3) -> Self {
        let mut tr = Self::new();
        tr.translate(translate);
        tr
    }
    pub fn with_rotation(rotation: Vec3) -> Self {
        let mut tr = Self::new();
        tr.rotate(rotation);
        tr
    }
    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.position) * self.get_rotation_matrix()
    }
    pub fn get_rotation_matrix(&self) -> Mat4 {
        Mat4::from_rotation_x(self.rotation.x)
            * Mat4::from_rotation_y(self.rotation.y)
            * Mat4::from_rotation_z(self.rotation.z)
    }
    pub fn rotate(&mut self, rot: Vec3) {
        self.rotation += rot
    }
    pub fn translate(&mut self, translate: Vec3) {
        self.position += translate
    }
}
impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}
