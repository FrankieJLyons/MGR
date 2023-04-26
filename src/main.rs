use macroquad::prelude::*;

mod game;
use crate::game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Metal Gear Rusted".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await.unwrap();

    loop {
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        game.update().await;

        clear_background(PINK);
        game.draw();
        next_frame().await;
    }
}