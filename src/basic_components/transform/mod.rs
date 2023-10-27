use glam::*;
use specs::prelude::*;
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    rotation_matrix: Mat4,
}
impl Transform {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
            rotation: vec3(0.0, 0.0, 0.0),
            right: Vec3::X,
            up: Vec3::Y,
            forward: Vec3::Z,
            rotation_matrix: Mat4::IDENTITY,
        }
    }
    pub fn with_position(translation: Vec3) -> Self {
        let mut trans = Self::new();
        trans.translate(translation);
        trans
    }
    pub fn with_rotation(rotation: Vec3) -> Self {
        let mut trans = Self::new();
        trans.rotate(rotation);
        trans
    }
    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }
    pub fn rotate(&mut self, rotation: Vec3) {
        self.rotation += rotation;
        self.rotation_matrix = self.get_rotation_matrix();
        self.forward = Mat3::from_mat4(self.rotation_matrix) * self.forward;
        self.up = Mat3::from_mat4(self.rotation_matrix) * self.up;
        self.right = Mat3::from_mat4(self.rotation_matrix) * self.right;
    }
    pub(crate) fn get_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.position) * self.rotation_matrix
    }
    fn get_rotation_matrix(&self) -> Mat4 {
        Mat4::from_rotation_x(self.rotation.x.to_radians())
            * Mat4::from_rotation_y(self.rotation.y.to_radians())
            * Mat4::from_rotation_z(self.rotation.z.to_radians())
    }
}
impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}
