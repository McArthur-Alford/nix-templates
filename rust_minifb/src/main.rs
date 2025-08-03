use std::{f32::consts::PI, time::Instant};

use glam::{U8Vec3, USizeVec2, Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[derive(Clone, Debug)]
pub struct Buffer<T> {
    pub buff: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Buffer<T> {
    pub fn set(&mut self, pos: USizeVec2, val: T) {
        self.buff
            .get_mut(pos.x % self.width + self.width * pos.y)
            .map(|c| *c = val);
    }

    pub fn setf(&mut self, pos: Vec2, val: T) {
        self.set(pos.round().as_usizevec2(), val);
    }

    pub fn get(&mut self, pos: USizeVec2) -> T {
        self.buff
            .get(pos.x % self.width + self.width * pos.y)
            .cloned()
            .unwrap()
    }

    pub fn reset(&mut self, val: T) {
        self.buff = vec![val; self.width * self.height];
    }
}

pub fn rgb_from_u8(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    r << 16 | g << 8 | b
}

pub fn rgb_from_vec(rgb: U8Vec3) -> u32 {
    let (r, g, b) = (rgb.x as u32, rgb.y as u32, rgb.z as u32);
    r << 16 | g << 8 | b
}

fn main() {
    let mut buffer = Buffer {
        width: WIDTH,
        height: HEIGHT,
        buff: vec![U8Vec3::ZERO; WIDTH * HEIGHT],
    };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(240);
    let time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.reset(U8Vec3::ZERO);

        for x in 0..buffer.width {
            for y in buffer.height / 2..buffer.height {
                buffer.set((x, y).into(), (255, 0, 0).into());
            }
        }

        window
            .update_with_buffer(
                &buffer
                    .buff
                    .iter()
                    .map(|x| rgb_from_vec(*x))
                    .collect::<Vec<_>>(),
                WIDTH,
                HEIGHT,
            )
            .unwrap();
    }
}
