use crate::pos;

pub struct Room {
    pub pos: pos::Pos<i32>,
    pub width: u32,
    pub height: u32,
    pub is_main_room: bool,
    // pub id: String,
}
