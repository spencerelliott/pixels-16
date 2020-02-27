use std::io::prelude::*;

use pixels::Pixels;
use crate::constants::CLEAR_BUFFER;

/// Provides a simple interface to modify a framebuffer
pub struct Surface {
    pixels: Pixels
}

impl Surface {
    pub fn new(pixels: Pixels) -> Self {
        Self {
            pixels
        }
    }

    /// Clears the current framebuffer
    pub fn cls(&mut self) {
        self.pixels.get_frame().write(&CLEAR_BUFFER).unwrap();
    }

    /// Presents the current frame and prepares the next frame for rendering
    pub fn flip(&mut self) {
        self.pixels.render();
    }
}