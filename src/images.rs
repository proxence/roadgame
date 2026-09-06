use std::rc::Rc;

use macroquad::texture::{Texture2D, load_texture};

pub struct Images {
    road: Rc<Texture2D>,
    player: Rc<Texture2D>,
}

impl Images {
    pub async fn new() -> Self {
        let road = load_texture("./assets/images/road.png").await.unwrap();
        let player = load_texture("./assets/images/red-car.png").await.unwrap();
        Images {
            road: Rc::new(road),
            player: Rc::new(player),
        }
    }

    pub fn get_road(&self) -> Rc<Texture2D> {
        return self.road.clone();
    }

    pub fn get_player(&self) -> Rc<Texture2D> {
        return self.player.clone();
    }
}
