use crate::pos;

pub struct Room {
    pub pos: pos::Pos<i32>,
    pub width: u32,
    pub height: u32,
    // pub id: String,
}

impl Room {
    pub fn left(&self) -> i32 {
        self.pos.x - (self.width as i32 / 2)
    }

    pub fn right(&self) -> i32 {
        self.pos.x + (self.width as i32 / 2)
    }

    pub fn top(&self) -> i32 {
        self.pos.y + (self.height as i32 / 2)
    }

    pub fn bottom(&self) -> i32 {
        self.pos.y - (self.height as i32 / 2)
    }
}
