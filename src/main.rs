mod room;
mod dungeon;
mod pos;

fn main() {
    let dungeon = dungeon::generate_dungeon(20);
    dungeon.place_rooms()
}
