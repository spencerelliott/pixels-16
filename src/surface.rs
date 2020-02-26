use pixels::Pixels;

pub struct Surface {
    pixels: Pixels
}

impl Surface {
    pub fn new(pixels: Pixels) -> Surface {
        Surface {
            pixels
        }
    }

    pub fn flip(&self) {

    }
}