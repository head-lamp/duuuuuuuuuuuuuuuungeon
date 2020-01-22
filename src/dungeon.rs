use crate::room;
use crate::pos;
use std::collections::HashSet;

extern crate rand;
use rand::Rng;

const PI:f64 =  std::f64::consts::PI;

pub struct Dungeon {
    pub rooms: Vec<room::Room>,
    pub num_rooms: u32,
    pub perimeter_room: room::Room,
    pub height: u32,
    pub width: u32,
    pub max_room_height: u32,
    pub max_room_width: u32,
}

pub fn new(num_rooms: u32) -> Dungeon {
    let mut rooms: Vec<room::Room> = Vec::new();
    let mut perimeter_room = room::Room {
            pos: pos::Pos{ x: 0 as i32, y: 0 as i32},
            width: 0,
            height: 0,
    };
    let mut dungen = Dungeon {
        rooms: rooms,
        perimeter_room: perimeter_room,
        num_rooms: num_rooms,
        max_room_height: 500, // TODO FIXME don't hardcode
        max_room_width: 500, // TODO FIXME don't hardcode
        width: 0,
        height: 0,
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
        let edges = self.get_edges();

        // basically create walls around the wall
        self.make_perimeter_room();
    }

    fn load_rooms(&mut self) {
        let mut rng = rand::thread_rng();

        for _ in 0u32..self.num_rooms {
            let pos = rand_point_in_circle(80);
            let w = rng.gen_range(0, self.max_room_width);
            let h = rng.gen_range(0, self.max_room_height);
            let mut room = room::Room {
                pos: pos,
                width: w,
                height: h,
            };
            self.rooms.push(room)
        }
    }

    fn get_dist_between_rooms(&self, i:usize, j:usize) -> u32{
        let a = &self.rooms[i];
        let b = &self.rooms[j];
        ((a.pos.x - b.pos.x).pow(2) + (a.pos.y - b.pos.y).pow(2)).abs() as u32
    }

    fn get_edges(&self) -> HashSet::<(usize, usize)> {
        let mut edges = HashSet::<(usize, usize)>::new();
        for i in 0..self.rooms.len() {
            let mut min_dist = 99999999;
            let mut idx = i;
            for j in 0..self.rooms.len() {
                if i == j {
                    continue;
                }

                let dist = self.get_dist_between_rooms(i as usize, j as usize);
                let hasEdge = edges.contains(&(i, j)) || edges.contains(&(j,i));
                if !hasEdge && dist < min_dist {
                    min_dist = dist;
                    idx = j;
                }
            }
            edges.insert((i,idx));
        }
        return edges;
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

    fn make_perimeter_room(&mut self) {
        let mut right = 0;
        let mut top = 0;
        let mut left = 99999;
        let mut bottom = 99999;
        for room in &self.rooms {
            if room.right() > right {
                right = room.right();
            }
            if room.left() < left {
                left = room.left();
            }
            if room.top() > top {
                top = room.top();
            }
            if room.bottom() < bottom {
                bottom = room.bottom();
            }
        }

        self.perimeter_room.pos.x = (right + left) / 2; 
        self.perimeter_room.pos.y = (top + bottom) / 2;
        self.perimeter_room.width = (left - right).abs() as u32 + 2;
        self.perimeter_room.height = (top - bottom).abs() as u32 + 2;
    }
}
