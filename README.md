# Pixels-16

This library is meant to provide a simple 512x512 drawing surface for prototyping or making simple games.

## Features

* 512 x 512 framebuffer
* Pre-defined input with 4 directions and 4 input buttons (A, B, X, Y)
* WAV audio and sounds
* 1024 x 1024 customizable "video memory"

## TODO

- [ ] Customizable input key defintion
- [ ] Audio
- [ ] `Surface` video memory
- [ ] Finalized `Surface` API

## Getting Started

### Setting up a game
Pixels-16 requires a `struct` that implements the `Game` trait. The `Game` trait contains two methods, `on_update` and `on_draw`. Below is a simple example:

```rust
struct FunGame;

impl Game for FunGame {
    fn on_update(&mut self, delta: f32, input: &Input) {

    }

    fn on_draw(&mut self, surface: &mut Surface) {
        surface.cls();
    
        surface.flip();
    }
}
```

`FunGame` will clear the drawing surface and present the image on every frame.

### Running the game

To run your game, a new instance of `Pixels16` needs to be created and the `run` method needs to be called. The library will take care of the rest.

```rust
    let game = FunGame {};

    match Pixels16::new("Test game", Box::new(game)) {
        Ok(p16) => p16.run(),
        _ => {
            // Couldn't create the window successfully
        }
    }
```

## Handling Input

The `Input` class provides a simple interface to retrieve the status of pre-defined keys (defined in the `Keys` enum). `Pixels-16` supports 4 directions (up, down, left, right) and 4 buttons (A, B, X, Y). A reference to the state of input is passed into the `on_update` method of the currently running game.

```rust
struct FunGame;

impl Game for FunGame {
    fn on_update(&mut self, delta: f32, input: &Input) {
        if input.pressed(Key::UP) {
            // Move character upwards
        } else if input.pressed(Key::DOWN) {
            // Move character downwards
        }
    }

    fn on_draw(&mut self, surface: &mut Surface) {
        
    }
}
```