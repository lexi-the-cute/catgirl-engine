use sdl2::rect::{Point, Rect};

use super::entity::Entity;
// use crate::game::entity::entity::Entity;

pub struct Player {
    pub position: Point,
    pub sprite: Rect
}

impl Entity for Player {
    fn get_position(&self) -> Point {
        return self.position;
    }

    fn get_sprite(&self) -> Rect {
        return self.sprite;
    }

    // fn to_string(&self) -> String {
    //     return "Player".to_string();
    // }
}