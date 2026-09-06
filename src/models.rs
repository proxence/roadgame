use std::rc::Rc;

use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
    window::{screen_height, screen_width},
};

use crate::consts::{CAR_SIZE, PLAYER_SPEED, ROAD_PADDING, ROAD_SIZE, ROAD_SPEED};

struct CurrentPosition {
    x: f32,
    y: f32,
}

pub struct Road {
    image: Rc<Texture2D>,
    y1: f32,
    y2: f32,
}

impl Road {
    pub fn new(image: Rc<Texture2D>) -> Self {
        Road {
            image: image,
            y1: screen_height() * -1.,
            y2: 0.,
        }
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            screen_width() / 2. - ROAD_SIZE / 2.,
            self.y1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(ROAD_SIZE, screen_height())),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.image,
            screen_width() / 2. - ROAD_SIZE / 2.,
            self.y2,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(ROAD_SIZE, screen_height())),
                ..Default::default()
            },
        );

        if self.y2 >= screen_height() {
            self.y2 = 0.;
        } else {
            self.y2 += ROAD_SPEED;
        }

        self.y1 = ((screen_height() - self.y2) * -1.) + 2.;
    }

    /// Returns min_x, max_x.
    pub fn get_x_bounds(&self) -> (f32, f32) {
        let left_road_edge = screen_width() / 2. - ROAD_SIZE / 2.;
        let right_road_edge = left_road_edge + ROAD_SIZE;

        let min_x = left_road_edge + ROAD_PADDING;
        let max_x = right_road_edge - ROAD_PADDING - CAR_SIZE;

        (min_x, max_x)
    }

    /// Returns min_y, max_y.
    pub fn get_y_bounds(&self) -> (f32, f32) {
        let min_y = 0.;
        let max_y = screen_height() - (CAR_SIZE / 2.);
        (min_y, max_y)
    }
}

pub struct Player {
    image: Rc<Texture2D>,
    curr_pos: CurrentPosition,
}

impl Player {
    pub fn new(image: Rc<Texture2D>) -> Self {
        let curr_pos = CurrentPosition {
            x: screen_width() / 2.,
            y: screen_height() - 50.,
        };

        Player {
            curr_pos: curr_pos,
            image: image,
        }
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.curr_pos.x,
            self.curr_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(CAR_SIZE, CAR_SIZE)),
                ..Default::default()
            },
        );
    }

    pub fn left(&mut self, road: &Road) {
        let (min_x, max_x) = road.get_x_bounds();

        self.curr_pos.x = (self.curr_pos.x - PLAYER_SPEED).clamp(min_x, max_x);
    }

    pub fn right(&mut self, road: &Road) {
        let (min_x, max_x) = road.get_x_bounds();

        self.curr_pos.x = (self.curr_pos.x + PLAYER_SPEED).clamp(min_x, max_x);
    }

    pub fn up(&mut self, road: &Road) {
        let (min_y, max_y) = road.get_y_bounds();

        self.curr_pos.y = (self.curr_pos.y - PLAYER_SPEED).clamp(min_y, max_y)
    }

    pub fn down(&mut self, road: &Road) {
        let (min_y, max_y) = road.get_y_bounds();

        self.curr_pos.y = (self.curr_pos.y + PLAYER_SPEED).clamp(min_y, max_y)
    }
}
