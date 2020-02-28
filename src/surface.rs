use std::io::prelude::*;

use pixels::Pixels;
use crate::constants::{CLEAR_BUFFER, BUFFER_SIZE};

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

    fn extract_color(rgba: u32) -> (u8, u8, u8, u8) {
        (
            ((rgba & 0xFF_00_00_00) >> 24) as u8,
            ((rgba & 0x00_FF_00_00) >> 16) as u8,
            ((rgba & 0x00_00_FF_00) >> 8) as u8,
            (rgba & 0x00_00_00_FF) as u8,
        )
    }
    
    fn get_current_frame(&mut self) -> &mut [u8] {
        self.pixels.get_frame()
    }

    /// Clears the current framebuffer
    pub fn cls(&mut self) {
        self.pixels.get_frame().write(&CLEAR_BUFFER).unwrap();
    }

    pub fn blit_rect(&mut self) {

    }

    /// Draws a line between two points
    /// 
    /// # Arguments
    /// 
    /// * `x1` - The x position of the first point
    /// * `y1` - The y position of the first point
    /// * `x2` - The x position of the second point
    /// * `y2` - The y position of the second point
    /// * `rgba` - The RGBA representation of the color as a 32-bit integer (8-bits per color; `0xRRGGBBAA`)
    pub fn line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, rgba: u32) {

    }

    /// Sets a specific pixel to an RGBA value
    /// 
    /// # Arguments
    /// 
    /// * `x` - The x position of the pixel
    /// * `y` - The y position of the pixel
    /// * `rgba` - The RGBA representation of the color as a 32-bit integer (8-bits per color; `0xRRGGBBAA`)
    pub fn set(&mut self, x: u16, y: u16, rgba: u32) {

    }

    /// Presents the current frame and prepares the next frame for rendering
    pub fn flip(&mut self) {
        self.pixels.render();
    }
}