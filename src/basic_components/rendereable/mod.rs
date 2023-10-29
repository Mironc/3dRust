use corrosed_graphic::data_prelude::*;
use corrosed_graphic::geometry_prelude::*;
use once_cell::sync::Lazy;

use specs::prelude::*;
use std::collections::HashMap;

use std::sync::Arc;
use std::sync::Mutex;

use super::{camera::CameraMatrixes, light::Light, transform::Transform};
pub static PROGRAM_BUFFER: Lazy<Mutex<HashMap<u32, Program>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub struct MeshRenderer {
    pub program: u32,
    pub mesh: Arc<Mesh>,
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
            let mut program = PROGRAM_BUFFER.lock().unwrap();
            let program = program.get_mut(&renderer.program).unwrap();
            program.set_self();
            program.set_matrix4("view", false, cam_mat.view);
            program
                .set_matrix4("projection", false, cam_mat.projection)
                .unwrap();
            program
                .set_matrix4("model", false, trans.get_matrix())
                .unwrap();
            program.set_vec3("light_pos", light.pos);
            renderer.mesh.draw();
        }
    }
}
