use std::default::Default;
use pixels::wgpu;
use pixels::wgpu::{Backends, BlendState};
use crate::engine::{Engine, EngineBuilder, Game, RendererConfigure};

impl<'a, G> EngineBuilder<'a, G>
where G: Game + 'static{
    pub fn new(title: &'a str) -> Self {
        Self{
            title,
            application: None,
            width: 800,
            height: 600,
            renderer_configure: RendererConfigure::default(),
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn game(mut self, game: G) -> Self {
        self.application = Some(game);
        self
    }
    
    pub fn renderer_builder(mut self, renderer_configure: RendererConfigure) -> Self {
        self.renderer_configure = renderer_configure;
        self
    }

    pub fn build(self) -> anyhow::Result<Engine<G>> {
        if self.application.is_none() {
            panic!("No game specified; Please used [builder.game( * your code *)].");
        }

        Engine::new(self.title, self.width, self.height, self.application.unwrap(), self.renderer_configure)
    }

}

impl Default for RendererConfigure{
    fn default() -> Self {
        Self{
            clear_color: wgpu::Color::WHITE,
            vsync: true,
            wgpu_backend: Backends::VULKAN,
            blend_state: BlendState::ALPHA_BLENDING,
        }
    }
}