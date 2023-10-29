use specs::prelude::*;

use super::{camera::Camera, transform::Transform};
pub struct FlyCameraSys {}
impl<'a> System<'a> for FlyCameraSys {
    type SystemData = (ReadStorage<'a, Camera>, ReadStorage<'a, Transform>);

    fn run(&mut self, (camera, trans): Self::SystemData) {
        for (_, _transform) in (&camera, &trans).join() {}
    }
}
