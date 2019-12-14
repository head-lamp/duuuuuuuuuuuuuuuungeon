use crate::room;
use crate::pos;

extern crate rand;
use rand::Rng;

const PI:f64 =  std::f64::consts::PI;

pub struct Dungeon {
    pub rooms: Vec<room::Room>,
    pub num_rooms: u32,
    pub max_height: u32,
    pub max_width: u32,
}

pub fn new(num_rooms: u32) -> Dungeon {
    let mut rooms: Vec<room::Room> = Vec::new();
    let mut dungen = Dungeon {
        rooms: rooms,
        num_rooms: num_rooms,
        max_height: 500, // TODO FIXME don't hardcode
        max_width: 500, // TODO FIXME don't hardcode
    };

    return dungen
}

fn rand_point_in_circle(radius: u32) -> pos::Pos::<i32> {
    let mut rng = rand::thread_rng();
    let t = 2.0 * PI * rng.gen::<f64>();
    let u = rng.gen::<f64>() + rng.gen::<f64>();
    let r = if u > 1.0 {
        2.0 - u
    }
    else {
        u
    };

    let x = (radius as f64 * r * t.cos()) as i32;
    let y = (radius as f64 * r * t.sin()) as i32;
    pos::Pos{x: x, y: y}
}

impl Dungeon {
    pub fn generate(&mut self) {
        self.load_rooms();
        self.place_rooms();
    }

    fn load_rooms(&mut self) {
        let mut rng = rand::thread_rng();

        for _ in 0u32..self.num_rooms {
            let pos = rand_point_in_circle(80);
            let w = rng.gen_range(0, self.max_width);
            let h = rng.gen_range(0, self.max_height);
            let mut room = room::Room {
                pos: pos,
                width: w,
                height: h,
                is_main_room: false,
            };
            self.rooms.push(room)
        }
    }

    fn place_rooms(&mut self) {
        let mut done = false;
        while !done {
            let collisions = self.scatter_rooms();
            println!("collisions this round {}", collisions);
            if collisions == 0 {
                done = true;
            }
        }
    }

    fn room_overlap(&self, i: usize, j:usize) -> bool{
        let a = &self.rooms[i];
        let b = &self.rooms[j];

        if a.left() > b.right() || b.left() > a.left() {
            return false
        }

        if a.top() < b.bottom() || b.top() < a.bottom() {
            return false
        }

        true
    }

    fn repulse_room(&mut self, c: usize, e: usize) {
        let mut rng = rand::thread_rng();

        let a = &self.rooms[c].pos;
        let b = &self.rooms[e].pos;

        let x = a.x - b.x;
        let y = a.y - b.y;

        let mut pos = &mut self.rooms[c].pos;
        pos.x += x + rng.gen_range(-10, 10) * rng.gen_range(-1, 1);
        pos.y += y + rng.gen_range(-10, 10) * rng.gen_range(-1, 1);
    }

    fn scatter_rooms(&mut self) -> u32{
        let mut collisions = 0;
        for i in 0..self.rooms.len() {
            for j in 0..self.rooms.len() {
                if i == j {
                    continue;
                }
                if self.room_overlap(i as usize, j as usize) {
                    collisions += 1;
                    self.repulse_room(i, j);
                }
            }
        }
        collisions
    }
}
