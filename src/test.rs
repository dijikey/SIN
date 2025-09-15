use zxcmath::Vector2;
use crate::engine::{input, EngineBuilder, Game, State};
use crate::graphics::Color;

// mod lib;
// mod engine;
// mod graphics;

fn main() -> anyhow::Result<()> {
    let engine = EngineBuilder::new("My Game")
        .game(MyGame{ pos: Vector2::new(100.0, 100.0) })
        .size(800, 600)
        .build()?;

    engine.run()
}

struct MyGame{
    pos: Vector2
}

impl Game for MyGame{
    fn update(&mut self, state: &mut State){

    }
    fn draw(&mut self, state: &mut State){
        state.render.draw_line(self.pos, Vector2::new(120.0, 240.0), Color::BLACK);
    }

    fn key_released(&mut self, _keycode: u32) {

    }

    fn key_pressed(&mut self, keycode: u32) {
        if keycode == input::D {
            self.pos.x += 1.5;
        }
        if keycode == input::A {
            self.pos.x -= 1.5;
        }
        if keycode == input::W {
            self.pos.y -= 1.5;
        }
        if keycode == input::S {
            self.pos.y += 1.5;
        }
    }
}