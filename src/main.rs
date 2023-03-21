use macroquad::prelude::*;

mod game;
use crate::game::Game;

#[macroquad::main("Metal Gear Rusted")]
async fn main() {
    let mut game = Game::new().await.unwrap();
    loop {
        game.update();

        clear_background(BLACK);
        (game).draw();
        next_frame().await
    }
}
