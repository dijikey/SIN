<div align="center">

<img src="resource/icon.svg" width="256" />

A simple framework you can use to make 2D games, based on Rust and focused on simplicity

__SIN is a very raw project, so it may contain bugs and lack some functionality.__
</div>

## Usage
Add `sin-2d` as a dependency in your `Cargo.toml`
```toml
sin-2d = "*"
```

__If__ you have any performance issues, __try__ optimization level 2.

```toml
[profile.dev]
opt-level = 2
```

## Overview
An example of using __SIN__ to create a window:

```rust
use crate::engine::{EngineBuilder, Game, State};

fn main() -> Result<()> {
  let engine = EngineBuilder::new("My Game")
          .game(MyGame{  })
          .size(800, 600)
          .build()?;
  
  /*
    Logic before starting the game.
  */

  engine.run();
}

struct MyGame {  }

impl Game for MyGame {
  fn update(&mut self, state: &mut State){
    /*
    Logic on update game.
    */
  }
  fn draw(&mut self, state: &mut State){
    /*
    Logic on draw game, 
        you can use `state.render` for draw a primitive figures.
    */
  }
}
```

[//]: # (Browse the [documentation] and the [examples] to learn more!)

## Implementation details:
* [`winit`] window handling library in pure Rust.
* [`pixels`] a tiny hardware-accelerated pixel frame buffer. 🦀.
* [`zxcmath`] for `Vector`, and other math types.
* [`anyhow`] flexible concrete Error type built on std::error::Error

[//]: # (* [`image`] for image loading and texture array building.)

[`zxcmath`]: https://github.com/dijikey/zxcmath
[`winit`]: https://github.com/rust-windowing/winit
[`pixels`]: https://github.com/parasyte/pixels
[`anyhow`]: https://github.com/dtolnay/anyhow

[//]: # ([`image`]: https://github.com/image-rs/image)

## Feedback

This is my first serious Rust project, 
which I'm not good at anyway. 
So if it's possible to make the code cleaner and more user-friendly, 
then I'll be happy to accept help.

## Credits / Thank you
* [LÖVE], When creating the engine, I used the awesome LÖVE 2D framework as a basis.

[LÖVE]: https://github.com/love2d/love
