mod constants;
pub mod input;
pub mod surface;

use constants::{SURFACE_HEIGHT, SURFACE_WIDTH};

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::time::Instant;

pub trait Game {
    fn on_update(&mut self, delta: f32, input: &input::Input);
    fn on_draw(&mut self, surface: &mut surface::Surface);
}

pub struct Pixels16<G: Game + 'static> {
    surface: surface::Surface,
    input: input::Input,
    game: Box<G>,
    event_loop: EventLoop<()>,
    winit_input: WinitInputHelper,
    window: winit::window::Window,
}

impl<G: Game> Pixels16<G> {
    /// Constructs a new `Pixel16` instance with the specified title and game
    /// 
    /// # Arguments
    /// 
    /// * `window_title` - The text to display in the title bar of the window
    /// * `game` - A struct that implements the `Game` trait
    pub fn new(window_title: &'static str, game: Box<G>) -> Result<Pixels16<G>, Error> {
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(SURFACE_WIDTH as f64, SURFACE_HEIGHT as f64);
            WindowBuilder::new()
                .with_title(window_title)
                .with_inner_size(size)
                .with_resizable(false)
                .build(&event_loop)
                .unwrap()
        };

        let pixels = {
            let surface = Surface::create(&window);
            let surface_texture = SurfaceTexture::new(
                SURFACE_WIDTH,
                SURFACE_HEIGHT,
                surface,
            );
            Pixels::new(SURFACE_WIDTH, SURFACE_HEIGHT, surface_texture)?
        };

        let surface = surface::Surface::new(pixels);
        let input = input::Input::new();

        Ok(Pixels16 {
            surface,
            input,
            game,
            event_loop,
            winit_input: WinitInputHelper::new(),
            window,
        })
    }

    pub fn run(self) {
        let mut local_winit_input = self.winit_input;
        let mut local_surface = self.surface;
        let mut local_input = self.input;

        let local_window = self.window;

        let mut last_frame = Instant::now();

        let mut local_game = self.game;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(_) => {
                    let delta_time = (Instant::now() - last_frame).as_micros() as f32 / 1000.0;

                    local_game.on_update(delta_time, &local_input);
                    local_game.on_draw(&mut local_surface);

                    last_frame = Instant::now();
                }
                _ => {}
            }

            if local_winit_input.update(event) {
                if local_winit_input.key_pressed(VirtualKeyCode::Escape) || local_winit_input.quit()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Directions
                local_input.update(
                    input::Key::UP,
                    local_winit_input.key_held(VirtualKeyCode::Up),
                );
                local_input.update(
                    input::Key::DOWN,
                    local_winit_input.key_held(VirtualKeyCode::Down),
                );
                local_input.update(
                    input::Key::LEFT,
                    local_winit_input.key_held(VirtualKeyCode::Left),
                );
                local_input.update(
                    input::Key::RIGHT,
                    local_winit_input.key_held(VirtualKeyCode::Right),
                );

                // Buttons
                local_input.update(input::Key::A, local_winit_input.key_held(VirtualKeyCode::Z));
                local_input.update(input::Key::B, local_winit_input.key_held(VirtualKeyCode::X));
                local_input.update(input::Key::X, local_winit_input.key_held(VirtualKeyCode::A));
                local_input.update(input::Key::Y, local_winit_input.key_held(VirtualKeyCode::Y));

                local_window.request_redraw();
            }
        });
    }
}
