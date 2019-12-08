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

pub fn generate_dungeon(num_rooms: u32) -> Dungeon {
    let mut rooms: Vec<room::Room> = Vec::new();
    let dungen = Dungeon {
        rooms: rooms,
        num_rooms: num_rooms,
        max_height: 500,
        max_width: 500,
    };

    return dungen
}

fn rand_point_in_circle(radius: f64) -> pos::Pos::<i32> {
    let mut rng = rand::thread_rng();
    let t = 2.0 * PI * rng.gen::<f64>();
    let u = rng.gen::<f64>() + rng.gen::<f64>();
    let r = if u > 1.0 {
        2.0 - u
    }
    else {
        u
    };

    let x = (radius * r * t.cos()) as i32;
    let y = (radius * r * t.sin()) as i32;
    pos::Pos{x: x, y: y}
}

impl Dungeon {
    pub fn place_rooms(self) {
        for mut x in 0u32..self.num_rooms {
            println!("x = {}", x);
            let pos = rand_point_in_circle(80.0);
        }
    }
}
