use sdl2::rect::{Point, Rect};

struct EntityStruct {
    pub position: Point,
    pub sprite: Rect
}

pub trait Entity {
    fn get_position(&self) -> Point;
    fn get_sprite(&self) -> Rect;
    fn to_string(&self) -> String {
        return "Entity".to_string();
    }
}

impl Entity for EntityStruct {
    fn get_position(&self) -> Point {
        return self.position;
    }

    fn get_sprite(&self) -> Rect {
        return self.sprite;
    }

    fn to_string(&self) -> String {
        return "Player".to_string();
    }
}