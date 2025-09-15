use crate::engine::renderer::RendererSystem;
use crate::graphics::Color;
use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub mod renderer;
#[allow(dead_code)]
pub mod input;
#[allow(dead_code)]
mod builder;

pub struct State{
    pub render: RendererSystem
}

pub struct Engine<G>
where G: Game,{
    event_loop: Option<EventLoop<()>>,
    window: Window,
    state: State,
    application: G,
}

impl<G> Engine<G>
where G: Game + 'static{
    fn new(title: &str, width: u32, height: u32, application: G) -> Result<Self> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)?;

        let render = RendererSystem::new(&window, width, height)?;

        let state = State{
            render
        };

        Ok(Self {
            event_loop: Some(event_loop),
            window,
            state,
            application
        })
    }

    pub fn run(mut self) -> Result<()> {
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::RedrawRequested(..) => {
                    self.application.update(&mut self.state);
                    self.render().expect("Render failed");
                },
                Event::WindowEvent { event, .. } => {
                    match event {
                        // # SYSTEM CALLBACKS
                        WindowEvent::Resized(size) => {
                            self.state.render.resize(size.width, size.height).expect("Error when resizing the window");
                        }
                        WindowEvent::Moved(_) => {}
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        // # INPUT CALLBACKS
                        // CURSOR
                        WindowEvent::CursorMoved { .. } => {}
                        WindowEvent::CursorEntered { .. } => {}
                        WindowEvent::CursorLeft { .. } => {}
                        // MOUSE
                        WindowEvent::MouseWheel { .. } => {}
                        WindowEvent::MouseInput { .. } => {}
                        // KEYBOARD
                        WindowEvent::KeyboardInput { input, .. } => {
                            if input.state == ElementState::Pressed {
                                self.application.key_pressed(input.scancode);
                            }else {
                                self.application.key_released(input.scancode);
                            }
                        }

                        _ => {}
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        })
    }

    fn render(&mut self) -> Result<()> {
        self.state.render.clear(Color::WHITE);

        self.application.draw(&mut self.state);

        self.state.render.render()?;
        Ok(())
    }
}

pub struct EngineBuilder<'a, G>
    where G: Game {
    application: Option<G>,
    title: &'a str,
    width: u32,
    height: u32
}

pub trait Game{
    /// Each frame is called, used to update the game state.
    /// Called before rendering
    fn update(&mut self, state: &mut State);
    /// Each frame is called and used to draw the frame
    fn draw(&mut self, state: &mut State);
    /// Called if the button has been released
    fn key_released(&mut self, _keycode: u32){ }
    /// Called if the button has been pressed
    fn key_pressed(&mut self, _keycode: u32){ }
}