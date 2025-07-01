mod game;

mod render;
mod utils;
mod consts;
use consts::{SPRITE_SIZE, TILE_SIZE};

use game::Game;

#[macroquad::main("Rust Chess")]
async fn main() {
    let mut game = Game::new().await;
    game.run().await;
}