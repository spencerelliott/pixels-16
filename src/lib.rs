mod constants;
mod surface;
mod input;

use constants::{SURFACE_WIDTH, SURFACE_HEIGHT};

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub trait EventRunner {
    fn on_update(self, delta: f32, input: &WinitInputHelper);
    fn on_draw(self, surface: &mut surface::Surface);
}

pub struct Pixels16<'a> {
    surface: surface::Surface,
    input: input::Input,
    event_runner: &'a dyn EventRunner,
    event_loop: EventLoop<()>,
    winit_input: WinitInputHelper,
    window: winit::window::Window
}

impl<'a> Pixels16<'a> {
    pub fn new(window_title: &'static str, event_runner: &'a dyn EventRunner) -> Result<Pixels16<'a>, Error> {
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(SURFACE_WIDTH as f64, SURFACE_HEIGHT as f64);
            WindowBuilder::new()
                .with_title(window_title)
                .with_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut scale_factor = window.scale_factor();

        let pixels = {
            let surface = Surface::create(&window);
            let surface_texture = SurfaceTexture::new(SURFACE_WIDTH, SURFACE_HEIGHT, surface);
            Pixels::new(SURFACE_WIDTH, SURFACE_HEIGHT, surface_texture)?
        };

        let surface = surface::Surface::new(pixels);
        let input = input::Input::new();

        Ok(Pixels16 {
            surface,
            input,
            event_runner,
            event_loop,
            winit_input: WinitInputHelper::new(),
            window
        })
    }

    pub fn run(self) {
        let mut local_winit_input = self.winit_input;
        let mut local_surface = self.surface;
        let mut local_input = self.input;
        //let local_window = self.window;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(_) => {
                    //self.event_runner.on_draw(&mut local_surface);
                }
                _ => { }
            }

            if local_winit_input.update(event) {
                // Directions
                local_input.update(input::Key::UP, local_winit_input.key_held(VirtualKeyCode::Up));
                local_input.update(input::Key::DOWN, local_winit_input.key_held(VirtualKeyCode::Down));
                local_input.update(input::Key::LEFT, local_winit_input.key_held(VirtualKeyCode::Left));
                local_input.update(input::Key::RIGHT, local_winit_input.key_held(VirtualKeyCode::Right));

                // Buttons
                local_input.update(input::Key::A, local_winit_input.key_held(VirtualKeyCode::Z));
                local_input.update(input::Key::B, local_winit_input.key_held(VirtualKeyCode::X));
                local_input.update(input::Key::X, local_winit_input.key_held(VirtualKeyCode::A));
                local_input.update(input::Key::Y, local_winit_input.key_held(VirtualKeyCode::Y));

                //local_window.request_redraw();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
