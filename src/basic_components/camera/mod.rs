use std::sync::OnceLock;

use glam::Mat4;
use specs::prelude::*;
pub static MAINCAMERA: OnceLock<Camera> = OnceLock::new();
use super::transform::Transform;
pub struct Camera {
    pub fov: f32,
    aspect: f32,
    znear: f32,
    zfar: f32,
}
impl Camera {
    pub fn new(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        Self {
            fov,
            aspect,
            znear,
            zfar,
        }
    }
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
    //rn is useless because i using opengl
    pub fn build_perspective_infinite(&self) -> Mat4 {
        Mat4::perspective_infinite_lh(self.fov.to_radians(), self.aspect, self.znear)
    }
    pub fn build_perspective(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov.to_radians(), self.aspect, self.znear, self.zfar)
    }
    pub fn build_view(&self, transform: &Transform) -> Mat4 {
        Mat4::look_at_rh(
            transform.position,
            transform.position + transform.forward,
            transform.up,
        )
    }
}
impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}
pub struct CameraSys {}
impl<'a> System<'a> for CameraSys {
    type SystemData = (
        Write<'a, CameraMatrixes>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (mut cam_mat, cam, trans): Self::SystemData) {
        for (camera, transform) in (&cam, &trans).join() {
            cam_mat.projection = camera.build_perspective();
            cam_mat.view = camera.build_view(transform);
        }
    }
}
#[derive(Default)]
pub struct CameraMatrixes {
    pub projection: Mat4,
    pub view: Mat4,
}
impl CameraMatrixes {
    pub fn new() -> Self {
        Self {
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
        }
    }
}
