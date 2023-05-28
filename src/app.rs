use simple_logger::SimpleLogger;
use std::error::Error;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        SimpleLogger::default().env().init()?;

        log::debug!("Creating window and event loop");
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Spectral Renderer")
            .with_inner_size(PhysicalSize::new(800, 600))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state,
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    if let (VirtualKeyCode::Escape, ElementState::Pressed) = (keycode, state) {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                // On resize
                Event::WindowEvent {
                    event: WindowEvent::Resized(..),
                    ..
                } => {
                    log::debug!("Window has been resized");
                    // TODO: resize window
                }

                // Draw
                Event::MainEventsCleared => {
                    // TODO: draw
                }

                // Exit app on request to close window
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                // Wait for gpu to finish pending work before closing app
                Event::LoopDestroyed => {
                    // TODO: wait for gpu to finish pending work before closing
                }

                _ => {}
            }
        });
    }
}
