mod room;
mod dungeon;
mod pos;

fn main() {
    let mut dungeon = dungeon::new(20);
    dungeon.generate();
}
