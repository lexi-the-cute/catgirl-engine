use sdl2::rect::{Point, Rect};

// use super::entity::Entity;
// use crate::game::entity::entity::Entity;

pub struct Player {  // for Entity
    pub position: Point,
    pub sprite: Rect
}

// pub trait Player1 {
//     fn to_string(&self) -> String;
// }

impl Player {  // for Entity
    fn new(position: Point, sprite: Rect) -> Player {
        Player { 
            position: position,
            sprite: sprite
        }
    }

    pub fn get_position(&self) -> Point {
        return self.position;
    }

    pub fn get_sprite(&self) -> Rect {
        return self.sprite;
    }

    pub fn to_string(&self) -> String {
        return "Player".to_string();
    }
}