mod consts;
mod images;
mod models;

use macroquad::prelude::*;

use crate::{
    images::Images,
    models::{Player, Road},
};

struct Game {
    road: Road,
    player: Player,
    images: Images,
}

impl Game {
    async fn new() -> Self {
        let images = Images::new().await;
        Game {
            road: Road::new(images.get_road()),
            player: Player::new(images.get_player()),
            images: images,
        }
    }

    fn draw(&mut self) {
        self.road.draw();
        self.player.draw();
    }

    pub fn left(&mut self) {
        self.player.left(&self.road);
    }

    pub fn right(&mut self) {
        self.player.right(&self.road);
    }

    pub fn up(&mut self) {
        self.player.up(&self.road);
    }

    pub fn down(&mut self) {
        self.player.down(&self.road);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        events(&mut game);
        game.draw();

        next_frame().await
    }
}

fn events(game: &mut Game) {
    if is_key_down(KeyCode::Right) {
        game.right();
    }

    if is_key_down(KeyCode::Left) {
        game.left();
    }

    if is_key_down(KeyCode::Up) {
        game.up();
    }

    if is_key_down(KeyCode::Down) {
        game.down();
    }
}
