mod basic_components;
mod resources;
use std::alloc::GlobalAlloc;

use basic_components::{
    camera::{Camera, CameraMatrixes, CameraSys, MAINCAMERA}, /*
                                                             fly_camera::FlyCamera,
                                                             input::{Axis, Input}, */
    light::Light,
    rendereable::{MeshRenderSys, MeshRenderer},
    transform::Transform,
};
use corrosed_graphic::data_prelude::*;
use corrosed_graphic::geometry_prelude::*;
use glam::*;
use glfw::*;
use specs::prelude::*;
#[global_allocator]
static ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let mut glfw = init_no_callbacks().unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    if cfg!(target_os = "apple") {
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true))
    }
    let (mut window, events) = glfw
        .create_window(800, 800, "!", WindowMode::Windowed)
        .unwrap();
    window.make_current();
    window.set_key_polling(true); /*
                                  glfw.set_swap_interval(glfw::SwapInterval::Sync(0)); */
    /*
    let mut input = Input::new();
    input.add_axis(Axis::new("Horizontal", "a", "d"));
    input.add_axis(Axis::new("Vertical", "w", "s")); */
    gl::load_with(|x| window.get_proc_address(x));

    let mut world = World::new(); /*
                                  world.insert(input); */
    world.insert(CameraMatrixes::new());
    world.insert(Light {
        pos: vec3(0.5, 0.5, 0.),
    });
    world.register::<MeshRenderer>();
    world.register::<Transform>();
    world.register::<Camera>();
    let program = Program::from_shaders(
        Shader::from_source(include_str!("vert.glsl"), gl::VERTEX_SHADER),
        Shader::from_source(include_str!("frag.glsl"), gl::FRAGMENT_SHADER),
    );
    let model_mat = Transform::with_position(vec3(0., 0., 4.));
    let model = MeshRenderer {
        mesh: Mesh::from_str(include_str!("Cube.obj")),
        program: program.clone(),
    };
    world.create_entity().with(model).with(model_mat).build();
    let mut camera_trans = Transform::with_position(vec3(3., 1., 0.));
    camera_trans.rotate(vec3(0.0, 3.79, 0.0));
    let mut camera = Camera::new(
        60.,
        window.get_size().0 as f32 / window.get_size().1 as f32,
        0.01,
        1024.,
    ); /*
       MAINCAMERA.set(camera); */
    world
        .create_entity()
        .with(camera)
        .with(camera_trans)
        .build();
    let mut dispatcher = DispatcherBuilder::new()
        .with(CameraSys {}, "cam_sys", &[])
        .with_thread_local(MeshRenderSys {})
        .build();

    window.set_framebuffer_size_callback(|width, height| unsafe {
        gl::Viewport(0, 0, width, height);
    });

    unsafe {
        gl::Enable(gl::DEPTH_TEST); /*
                                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); */
    };
    let mut delta_time = 0.0;
    while !window.should_close() {
        {
            let start = glfw.get_time();
            /*
            let mut input = world.write_resource::<Input>(); */
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::Close => window.set_should_close(true), /*
                    WindowEvent::Key(key, _, action, _) => match action {
                    Action::Release => input.process_key_up(key),
                    Action::Press => input.process_key_down(key, true),
                    Action::Repeat => input.process_key_down(key, false),
                    }, */
                    _ => (),
                }
            } /*
              drop::<FetchMut<'_, Input<'_>>>(input); */
            unsafe {
                gl::ClearColor(0., 0., 0., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            dispatcher.dispatch(&world);
            window.swap_buffers(); /*
                                   let mut input = world.write_resource::<Input>();
                                   input.on_loop_end(); */
            glfw.poll_events();
            delta_time = glfw.get_time() - start;
            println!("{}", 1.0 / delta_time);
        }
    }
}
pub mod reg {
    #[macro_export]
    macro_rules! Register {
        ($world:item;$($x:expr), *) =>
        {
            $(world.register::<x>();)*
        };
    }
}
