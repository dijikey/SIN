use cloud::engine::{EngineBuilder, Game, State};
use cloud::graphics::{Color, Sprite};
use cloud::{anyhow, MouseButton, Vector2};
fn main() -> anyhow::Result<()> {
    let engine = EngineBuilder::new("SANDFALL")
        .game(MyGame{
            blocks: Vec::new(),
            mouse_pressed: false,
            sprite: Sprite::new(String::from("resource/sandfall/icon.png"))?,
            need_x: [500.0; 512],
        })
        .size(512, 512)
        .build()?;

    engine.run()
}

struct Block{
    color: Color,
    position: Vector2,
    speed: f64,
    frozen: bool,
}

struct MyGame{
    blocks: Vec<Block>,
    mouse_pressed: bool,
    sprite: Sprite,
    need_x: [f64; 512],
}

impl Game for MyGame{
    fn update(&mut self, state: &mut State){
        for block in &mut self.blocks {
            if block.position.x > 511.0 { block.position.x = 511.0; }
            if block.position.y < self.need_x[block.position.x as usize] && !block.frozen {
                block.position.y += block.speed;
                block.speed += 0.5;
                if block.position.y > self.need_x[block.position.x as usize] {
                    self.need_x[block.position.x as usize] -= 4.0;
                }
            }else {
                block.frozen = true;
            }
        }

        if self.mouse_pressed {
            self.blocks.push(Block{
                position: state.mouse_position,
                color: Color::from_rgb(rand::gen_mod_u64(255) as u8, rand::gen_mod_u64(255) as u8, rand::gen_mod_u64(255) as u8),
                speed: 1.0,
                frozen: false,
            });
        }
    }
    fn draw(&mut self, state: &mut State){
        state.render.draw_sprite(&Vector2::ZERO, &self.sprite);
        const BLOCK_SIZE: Vector2 = Vector2::new(4.0, 4.0);
        for block in &self.blocks {
            state.render.draw_rect(&block.position,&BLOCK_SIZE, &block.color);
        }
    }

    fn mouse_pressed(&mut self, _: Vector2, _: MouseButton) {
        self.mouse_pressed = true;
    }
    fn mouse_released(&mut self, _: Vector2,_: MouseButton) {
        self.mouse_pressed = false;
    }
}

/// Take from fastrand
mod rand{
    static mut SEED: u64 = 0;
    #[inline]
    fn gen_u64() -> u64 {
        const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
        const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;

        let s = unsafe { SEED }.wrapping_add(WY_CONST_0);
        unsafe { SEED = s; }
        let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
        (t as u64) ^ (t >> 64) as u64
    }

    #[inline]
    pub(crate) fn gen_mod_u64(n: u64) -> u64 {
        let mut r = gen_u64();
        let mut hi = mul_high_u64(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = gen_u64();
                hi = mul_high_u64(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }

    #[inline]
    fn mul_high_u64(a: u64, b: u64) -> u64 {
        (((a as u128) * (b as u128)) >> 64) as u64
    }
}