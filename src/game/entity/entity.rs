use sdl2::rect::{Point, Rect};

pub struct Entity {
    pub position: Point,
    pub sprite: Rect
}

impl Entity {
    fn new(position: Point, sprite: Rect) -> Entity {
        Entity { 
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
        return "Entity".to_string();
    }
}