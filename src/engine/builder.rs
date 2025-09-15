use crate::engine::{Engine, EngineBuilder, Game};

impl<'a, G> EngineBuilder<'a, G>
where G: Game + 'static{
    pub fn new(title: &'a str) -> Self {
        Self{
            title,
            application: None,
            width: 800,
            height: 600,
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

    pub fn build(self) -> anyhow::Result<Engine<G>> {
        if self.application.is_none() {
            panic!("No game specified; Please used [builder.game( * your code *)].");
        }

        Engine::new(self.title, self.width, self.height, self.application.unwrap())
    }

}