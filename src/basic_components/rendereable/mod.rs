use corrosed_graphic::data_prelude::*;
use corrosed_graphic::geometry_prelude::*;
use specs::prelude::*;

use super::{camera::CameraMatrixes, light::Light, transform::Transform};
pub struct MeshRenderer {
    pub program: Program,
    pub mesh: Mesh,
}
impl Component for MeshRenderer {
    type Storage = DenseVecStorage<Self>;
}
pub struct MeshRenderSys {}
impl<'a> System<'a> for MeshRenderSys {
    type SystemData = (
        Read<'a, CameraMatrixes>,
        ReadStorage<'a, MeshRenderer>,
        ReadStorage<'a, Transform>,
        Read<'a, Light>,
    );

    fn run(&mut self, (cam_mat, renderer, trans, light): Self::SystemData) {
        for (renderer, trans) in (&renderer, &trans).join() {
            renderer.program.set_self();
            renderer
                .program
                .set_matrix4("model", false, trans.get_matrix())
                .unwrap();
            renderer
                .program
                .set_matrix4("view", false, cam_mat.view)
                .unwrap();
            renderer
                .program
                .set_matrix4("projection", false, cam_mat.projection)
                .unwrap();
            renderer.program.set_vec3("light_pos", light.pos);
            renderer.mesh.draw();
        }
    }
}
