use glium::{glutin as gl, Surface, uniform};
use cgmath::Matrix4;
use crate::geometry::{vertex::Vertex, mesh::Mesh};
use std::time::Instant;
pub fn win_init() {
    let event_loop = gl::event_loop::EventLoopBuilder::new().build();
    let window = gl::window::WindowBuilder::new()
        .with_title("Game")
        .with_inner_size(gl::dpi::PhysicalSize::new(800, 800));
    let context = gl::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let program = glium::Program::from_source(&display, include_str!("vert.glsl"), include_str!("frag.glsl"), None).unwrap();
    let start = Instant::now();
    let cube_mesh = Mesh::from_str(include_str!("../cube.obj"), &display);
    println!("time costed to load model {}",(Instant::now()-start).as_micros());
    let shape_buff = glium::VertexBuffer::new(&display, &vec![Vertex::new([-0.5, -0.5,0.]),Vertex::new([0.0,  0.5,0.]),Vertex::new([0.5, -0.25,0.])]).unwrap();
    let ind_buff = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let mut delta_time =0.0;
    let mut rot: f32 = 0.0;
    event_loop.run(move|event, _, control_flow| 
        {
            match event 
            {
                gl::event::Event::WindowEvent { window_id, event } => 
                {
                    if window_id == display.gl_window().window().id()
                    {
                        match event {
                            gl::event::WindowEvent::Resized(new_size) => display.gl_window().resize(new_size),
                            gl::event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => display.gl_window().resize(*new_inner_size),
                            gl::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                            gl::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => (),
                            gl::event::WindowEvent::CursorMoved { device_id, position, modifiers } => (),
                            gl::event::WindowEvent::CursorEntered { device_id } => (),
                            gl::event::WindowEvent::CursorLeft { device_id } => (),
                            gl::event::WindowEvent::MouseInput { device_id, state, button, modifiers } => (),
                            _=>()
                        }
                    }
                },
                gl::event::Event::RedrawRequested(id) =>
                {
                    if id == display.gl_window().window().id()
                    {
                        rot += 0.02 * 0.5;
                        let rotate_mat = [
                            [rot.to_radians().cos(),0.0,-rot.to_radians().sin(),0.0],
                            [0.0,1.0,0.0,0.0],
                            [rot.to_radians().sin(),0.0,rot.to_radians().cos(),0.0],
                            [0.0,0.0,0.0,1.0f32],
                            ];
                        let mat = uniform! {matrix:matrix_into_array( Matrix4::from_scale(0.125) * Matrix4::from(rotate_mat))};
                        /* [
                            [rot.to_radians().cos(),0.0, -rot.to_radians().sin(), 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [rot.to_radians().sin(), 0.0, rot.to_radians().cos(), 0.0],
                            [0.0 , 0.0, 0.0, 1.0f32],
                        ] */
                        let mut frame = display.draw();
                        frame.clear_color(1., 1., 1., 1.);
                        
                        frame.draw(&cube_mesh.vertexes, &cube_mesh.indices, &program, &mat, &glium::draw_parameters::DrawParameters{..Default::default()}).unwrap();
                        frame.finish().unwrap();
                    }
                }
                gl::event::Event::RedrawEventsCleared =>
                {
                    display.gl_window().window().request_redraw();
                }
                _=>()
            }
        });
}
fn matrix_into_array(mat:Matrix4<f32>)-> [[f32;4];4]
{
    mat.into()
}
