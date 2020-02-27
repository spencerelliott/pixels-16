use std::io::prelude::*;

use pixels::Pixels;
use crate::constants::CLEAR_BUFFER;

pub struct Surface {
    pixels: Pixels
}

impl Surface {
    pub fn new(pixels: Pixels) -> Surface {
        Surface {
            pixels
        }
    }

    pub fn cls(&mut self) {
        self.pixels.get_frame().write(&CLEAR_BUFFER).unwrap();
    }

    pub fn flip(&mut self) {
        self.pixels.render();
    }
}