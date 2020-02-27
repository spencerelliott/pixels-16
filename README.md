# Pixels-16

This library is meant to provide a simple 512x512 drawing surface for prototyping or making simple games.

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