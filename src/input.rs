#[derive(Clone, Copy)]
pub enum Key {
    UP =    1,
    DOWN =  1 << 1,
    LEFT =  1 << 2,
    RIGHT = 1 << 3,
    A =     1 << 4,
    B =     1 << 5,
    X =     1 << 6,
    Y =     1 << 7
}

#[derive(Clone, Copy)]
pub struct Input {
    mask: u32
}

impl Input {
    pub fn new() -> Input {
        Input {
            mask: 0x0
        }
    }

    pub fn pressed(&self, key: Key) -> bool {
        self.mask & key as u32 == key as u32
    }

    pub fn update(&mut self, key: Key, enabled: bool) {
        if enabled {
            self.mask |= key as u32;
        } else {
            self.mask &= !(key as u32);
        }
    }
}