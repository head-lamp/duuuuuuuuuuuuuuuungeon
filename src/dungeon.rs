use crate::room::Room;

pub struct Dungeon {
    pub rooms: Vec<Room>,
    pub max_heigh: i32,
    pub max_width: i32,
}
