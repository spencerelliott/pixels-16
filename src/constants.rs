pub const SURFACE_HEIGHT: u32 = 512;
pub const SURFACE_WIDTH: u32 = 512;

pub const COLOR_DEPTH: u8 = 4;

pub const BUFFER_SIZE: usize = SURFACE_HEIGHT as usize * SURFACE_WIDTH as usize * COLOR_DEPTH as usize;

pub const CLEAR_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];