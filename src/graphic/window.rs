use cgmath::{Matrix4, perspective, Deg};
use glium::{glutin::{self as gl}, Surface, uniform};
use crate::{geometry::mesh::Mesh, camera};
use crate::input::Input;
use std::time::Instant;
pub fn win_init() {
    let mut camera = camera::Camera::new(45.,1024.,0.01,1.);
    let mut input_handler = Input::new();
    let event_loop = gl::event_loop::EventLoopBuilder::new().build();
    let context = gl::ContextBuilder::new();
    let window = gl::window::WindowBuilder::new()
        .with_title("Game")
        .with_inner_size(gl::dpi::PhysicalSize::new(800, 800));
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let program = glium::Program::from_source(&display, include_str!("vert.glsl"), include_str!("frag.glsl"), None).unwrap();
    let start = Instant::now();
    let mesh = Mesh::from_str(include_str!("../MonkeyHead.obj"), &display);
    println!("time costed to load model {}",(Instant::now()-start).as_micros());
    let light = [0.0, -1.0, 0.9f32];
    let mut delta_time =0.0;
    let mut rot: f32 = 0.0;
    let mut cam_y_rot = 0.;
    let mut cam_x_rot = 0.;
    let mut focused = false;
    event_loop.run(move|event, _, control_flow| 
        {
            display.gl_window().window().set_cursor_visible(false);
            match event 
            {
                gl::event::Event::WindowEvent { window_id, event } => 
                {
                    if window_id == display.gl_window().window().id()
                    {
                        match event 
                        {
                            gl::event::WindowEvent::Resized(new_size) => display.gl_window().resize(new_size),
                            gl::event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } =>{display.gl_window().resize(*new_inner_size);camera.set_aspect(new_inner_size.height, new_inner_size.width);} ,
                            gl::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                            gl::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic: _ } =>
                            {
                                match input.virtual_keycode {
                                    Some(keycode) => input_handler.handle_key_input(keycode,input.state),
                                    None => (),
                                }
                            } ,
                            gl::event::WindowEvent::CursorMoved { device_id, position, modifiers } => input_handler.handle_mouse_position(position.cast()),
                            gl::event::WindowEvent::MouseInput { device_id, state, button, modifiers } => (),
                            gl::event::WindowEvent::Focused(b) => focused = b,
                            _=>()
                        }
                    }
                },
                gl::event::Event::RedrawRequested(id) =>
                {
                    if id == display.gl_window().window().id()
                    {
                        if focused
                        {
                            let start = Instant::now();
                            let draw_params = glium::draw_parameters::DrawParameters
                            {
                                depth:glium::Depth 
                                { 
                                    test: glium::draw_parameters::DepthTest::IfLess,
                                    write: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            };
                            camera.yaw += input_handler.mouse_vel().x * delta_time ;
                            camera.pitch -=  input_handler.mouse_vel().y * delta_time ;
                            rot += 200. *delta_time;
                            let rotate_mat = [
                                [rot.to_radians().cos(),0.0,-rot.to_radians().sin(),0.0],
                                [0.0,1.0,0.0,0.0],
                                [rot.to_radians().sin(),0.0,rot.to_radians().cos(),0.0],
                                [0.0,0.0,0.0,1.0f32],
                                ];
                            camera.update();
                            let uniforms = uniform! {matrix:matrix_into_array( Matrix4::from_scale(1.) * Matrix4::from(rotate_mat)),u_light:light,perspective:camera.perspective,view:camera.projection};
                            let mut frame = display.draw();
                            frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
                            
                            frame.draw(&mesh.vertexes, &mesh.indices, &program, &uniforms, &draw_params).unwrap();
                            frame.finish().unwrap();
                            input_handler.clear_input();
                            delta_time = (Instant::now() - start).as_secs_f32();
                        }
                        
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
fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
} 