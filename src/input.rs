use std::borrow::Cow;

use cgmath::{vec2, Vector2};
use glium::glutin::{
    dpi::PhysicalPosition,
    event::{ElementState, VirtualKeyCode},
};
pub struct Input<'a> {
    keys_down: Vec<VirtualKeyCode>,
    keys_up: Vec<VirtualKeyCode>,
    axises: Vec<Axis<'a>>,
    prev_mouse_pos: PhysicalPosition<f32>,
    mouse_pos: PhysicalPosition<f32>,
}
impl Input<'_> {
    pub fn new() -> Self {
        Self {
            keys_down: Vec::new(),
            keys_up: Vec::new(),
            axises: vec![
                Axis::new("Vertical".to_string(), "w", "s"),
                Axis::new("Horizontal".to_string(), "d", "a"),
            ],
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            prev_mouse_pos: PhysicalPosition::new(0.0, 0.0),
        }
    }
    pub fn handle_mouse_position(&mut self, new_pos: PhysicalPosition<f32>) {
        println!("{}",new_pos.x);
        self.mouse_pos = new_pos
    }
    pub fn handle_key_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => self.keys_down.push(key),
            ElementState::Released => {
                self.keys_up.push(key);
                self.keys_down.swap_remove(self.keys_down.iter().position(|&x| x == key).unwrap());
            }
        }
    }
    pub fn get_key_down_str(&self, string_code: &str) -> bool {
        self.get_key_down(str_into_keycode(string_code))
    }
    pub fn get_key_down(&self, key: VirtualKeyCode) -> bool {
        for pressed_key in self.keys_down.iter() {
            if key == *pressed_key {
                return true;
            }
        }
        false
    }
    pub fn get_key_up_str(&self, string_code: &str) -> bool {
        self.get_key_up(str_into_keycode(string_code))
    }
    pub fn get_key_up(&self, key: VirtualKeyCode) -> bool {
        for key_up in self.keys_up.iter() {
            if key == *key_up {
                return true;
            }
        }
        false
    }
    pub fn mouse_vel(&self) -> Vector2<f32> {
        vec2(
            self.mouse_pos.x - self.prev_mouse_pos.x,
            self.mouse_pos.y - self.prev_mouse_pos.y,
        )
    }
    pub fn mouse_pos(&self) -> Vector2<f32> {
        vec2(self.mouse_pos.x, self.mouse_pos.y)
    }
    pub fn get_axis_raw(&self, axis_name: &str) -> f32 {
        match self.axises.iter().find(|&x| x.name == axis_name) {
            Some(axis) => axis.process(self),
            None => panic!("there is no axis {}", axis_name),
        }
    }
    pub fn clear_input(&mut self) {
        self.keys_up = Vec::new();
        self.prev_mouse_pos = self.mouse_pos;
    }
}
fn str_into_keycode(string: &str) -> VirtualKeyCode {
    match string.to_lowercase().as_str() {
        "1" => VirtualKeyCode::Key1,
        "2" => VirtualKeyCode::Key2,
        "3" => VirtualKeyCode::Key3,
        "4" => VirtualKeyCode::Key4,
        "5" => VirtualKeyCode::Key5,
        "6" => VirtualKeyCode::Key6,
        "7" => VirtualKeyCode::Key7,
        "8" => VirtualKeyCode::Key8,
        "9" => VirtualKeyCode::Key9,
        "0" => VirtualKeyCode::Key0,
        "a" => VirtualKeyCode::A,
        "b" => VirtualKeyCode::B,
        "c" => VirtualKeyCode::C,
        "d" => VirtualKeyCode::D,
        "e" => VirtualKeyCode::E,
        "f" => VirtualKeyCode::F,
        "g" => VirtualKeyCode::G,
        "h" => VirtualKeyCode::H,
        "i" => VirtualKeyCode::I,
        "j" => VirtualKeyCode::J,
        "k" => VirtualKeyCode::K,
        "l" => VirtualKeyCode::L,
        "m" => VirtualKeyCode::M,
        "n" => VirtualKeyCode::N,
        "o" => VirtualKeyCode::O,
        "p" => VirtualKeyCode::P,
        "q" => VirtualKeyCode::Q,
        "r" => VirtualKeyCode::R,
        "s" => VirtualKeyCode::S,
        "t" => VirtualKeyCode::T,
        "u" => VirtualKeyCode::U,
        "v" => VirtualKeyCode::V,
        "w" => VirtualKeyCode::W,
        "x" => VirtualKeyCode::X,
        "y" => VirtualKeyCode::Y,
        "z" => VirtualKeyCode::Z,
        _ => panic!("not valid keycode"),
    }
}
pub struct Axis<'a> {
    name: String,
    pos_key: Cow<'a, str>,
    neg_key: Cow<'a, str>,
}
impl<'a> Axis<'a> {
    pub fn new<S>(name: String, pos: S, neg: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            name,
            pos_key: pos.into(),
            neg_key: neg.into(),
        }
    }
    pub fn process(&self, input: &Input) -> f32 {
        match (
            input.get_key_down_str(&self.pos_key),
            input.get_key_down_str(&self.neg_key),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        }
    }
}
