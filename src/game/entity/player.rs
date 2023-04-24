// https://medium.com/comsystoreply/28-days-of-rust-part-2-composition-over-inheritance-cab1b106534a

use sdl2::rect::{Point, Rect};

// pub struct Inventory {

// }

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction
    // pub inventory: Inventory
}

impl Player {
    pub fn get_position(&self) -> Point {
        return self.position;
    }

    pub fn get_sprite(&self) -> Rect {
        return self.sprite;
    }
    
    // pub fn get_speed(&self) -> i32 {
    //     return self.speed;
    // }

    // pub fn set_position(&mut self, position: Point) {
    //     self.position = position;
    // }

    // pub fn set_sprite(&mut self, sprite: Rect) {
    //     self.sprite = sprite;
    // }

    // pub fn set_speed(&mut self, speed: i32) {
    //     self.speed = speed;
    // }

    // pub fn get_inventory(&self) -> Inventory {
    //     return self.inventory;
    // }

    pub fn update(&mut self) {
        use self::Direction::*;
        match self.direction {
            Left => {
                self.position = self.position.offset(-self.speed, 0);
            },
            Right => {
                self.position = self.position.offset(self.speed, 0);
            },
            Up => {
                self.position = self.position.offset(0, -self.speed);
            },
            Down => {
                self.position = self.position.offset(0, self.speed);
            },
            None => {
                self.position = self.position.offset(0, 0);
            }
            // _ => {
            //     // self.position = self.position.offset(0, 0);
            // }
        }
    }

    // pub fn to_string(&self) -> String {
    //     return "Player".to_string();
    // }
}