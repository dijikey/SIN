use std::path::Path;
use crate::engine::renderer::RendererSystem;
use crate::graphics::Color;
use anyhow::Result;
use libloading::Library;
use pixels::wgpu;
use pixels::wgpu::{Backends, BlendState};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use zxcmath::Vector2;

pub mod renderer;
#[allow(dead_code)]
pub mod input;
#[allow(dead_code)]
mod builder;

pub struct State{
    pub render: RendererSystem,
    pub mouse_position: Vector2,
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
    fn new(title: &str, width: u32, height: u32, application: G, renderer_configure: RendererConfigure) -> Result<Self> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)?;

        let render = RendererSystem::new(&window, width, height, renderer_configure)?;

        let state = State{
            render,
            mouse_position: Vector2::new(0.0, 0.0),
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
                            self.application.on_close();
                            *control_flow = ControlFlow::Exit;
                        }
                        // # INPUT CALLBACKS
                        // CURSOR
                        WindowEvent::CursorMoved { position, .. } => {
                            self.state.mouse_position.x = position.x;
                            self.state.mouse_position.y = position.y;
                        }
                        WindowEvent::CursorEntered { .. } => {}
                        WindowEvent::CursorLeft { .. } => {}
                        // MOUSE
                        WindowEvent::MouseWheel { .. } => {}
                        WindowEvent::MouseInput { button,state, .. } => {
                            match state {
                                ElementState::Pressed => {
                                    self.application.mouse_pressed(
                                        self.state.mouse_position,
                                        button,
                                    )
                                },
                                ElementState::Released => {
                                    self.application.mouse_released(
                                        self.state.mouse_position,
                                        button,
                                    )
                                }
                            }
                        }
                        // KEYBOARD
                        WindowEvent::KeyboardInput { input, .. } => {
                            match input.state {
                                ElementState::Pressed => {
                                    self.application.key_pressed(input.scancode);
                                },
                                ElementState::Released => {
                                    self.application.key_released(input.scancode);
                                }
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
        const CLEAR_COLOR: [u8; 4] = [255, 255, 255, 255];
        self.state.render.clear_screen(&CLEAR_COLOR);
        
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
    height: u32,
    renderer_configure: RendererConfigure
}
pub struct RendererConfigure{
    vsync: bool,
    clear_color: wgpu::Color,
    wgpu_backend: Backends,
    blend_state: BlendState
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
    /// Called if the mouse has been pressed
    fn mouse_pressed(&mut self, _position: Vector2, _button: MouseButton) { }
    /// Called if the mouse has been released
    fn mouse_released(&mut self, _position: Vector2, _button: MouseButton) { }
    /// Called when the application is closed
    fn on_close(&mut self){}
}