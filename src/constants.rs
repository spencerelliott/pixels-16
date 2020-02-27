pub const SURFACE_HEIGHT: u32 = 512;
pub const SURFACE_WIDTH: u32 = 512;

pub const BUFFER_SIZE: usize = SURFACE_HEIGHT as usize * SURFACE_WIDTH as usize;

pub const CLEAR_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];