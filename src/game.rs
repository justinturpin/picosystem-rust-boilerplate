use cxx::UniquePtr;
use alloc::vec;
use alloc::vec::Vec;
use super::ffi::*;
use super::Button;

static MOUNTAIN_BUFFER : &[u8] = super::include_bytes_align_as!(u16, "../assets/mountains.16bpp");

pub struct Game {
    mountain_buffer: UniquePtr<buffer_t>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mountain_buffer: buffer(120, 120, MOUNTAIN_BUFFER),
        }
    }

    pub fn update(&mut self, dt: u32) {

    }

    pub fn draw(&mut self) {
        // Turn off alpha blending to draw the background
        set_blend_alpha_enabled(false);

        blit(&self.mountain_buffer, 0, 0, 120, 120, 0, 0);

        set_blend_alpha_enabled(true);
    }
}
