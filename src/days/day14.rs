use std::io;

const MAP_LEN: i32 = 103;
const MAP_WIDTH: i32 = 101;
const NUM_ITER: usize = 100;

#[derive(Debug)]
struct Bot {
    pos: (usize, usize),
    vel: (i32, i32),
}

impl Bot {
    fn new(pos: (usize, usize), vel: (i32, i32)) -> Self {
        Self { pos, vel }
    }

    fn update_pos(&mut self) {
        let mut new_pos = (
            self.pos.0 as i32 + self.vel.0,
            self.pos.1 as i32 + self.vel.1,
        );

        // Check bounds
        // If going left
        if new_pos.0 < 0 {
            new_pos.0 = MAP_WIDTH + new_pos.0;
        }
        if new_pos.1 < 0 {
            new_pos.1 = MAP_LEN + new_pos.1;
        }

        // IF going right
        new_pos.0 = new_pos.0 % MAP_WIDTH;
        new_pos.1 = new_pos.1 % MAP_LEN;

        self.pos.0 = new_pos.0 as usize;
        self.pos.1 = new_pos.1 as usize;
    }
}

fn quad_count(bots: &Vec<Bot>) -> (i32, i32, i32, i32) {
    let mut quadrants = (0, 0, 0, 0);

    for el in bots.iter() {
        if el.pos.0 < MAP_WIDTH as usize / 2 {
            if el.pos.1 < MAP_LEN as usize / 2 {
                quadrants.0 += 1;
            } else if el.pos.1 > MAP_LEN as usize / 2 {
                quadrants.2 += 1;
            }
        } else if el.pos.0 > MAP_WIDTH as usize / 2 {
            if el.pos.1 < MAP_LEN as usize / 2 {
                quadrants.1 += 1;
            } else if el.pos.1 > MAP_LEN as usize / 2 {
                quadrants.3 += 1;
            }
        }
    }

    quadrants
}

fn print_map(bots: &Vec<Bot>) {
    let mut map = [[" "; MAP_WIDTH as usize]; MAP_LEN as usize];

    for b in bots.iter() {
        map[b.pos.1][b.pos.0] = "X";
    }

    for line in map.iter() {
        println!("{:?}", line);
    }
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut bots: Vec<Bot> = Vec::new();

    // Save all bots
    for line in file.lines() {
        let c: Vec<&str> = line.split(&[',', '=', ' ']).collect();

        bots.push(Bot::new(
            (
                c[1].parse::<usize>().expect("Err"),
                c[2].parse::<usize>().expect("Err"),
            ),
            (
                c[4].parse::<i32>().expect("Err"),
                c[5].parse::<i32>().expect("Err"),
            ),
        ));
    }

    // Part 1
    for _ in 0..NUM_ITER {
        for el in bots.iter_mut() {
            el.update_pos();
        }
    }

    let quadrants = quad_count(&bots);

    count.0 = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;

    // Check part 2
    let mut i = 1;
    loop {
        for el in bots.iter_mut() {
            el.update_pos();
        }

        let q = quad_count(&bots);

        println!("iter num {i}");
        i += 1;

        if q.0 < 100 {
            let mut buffer = String::new();
            print_map(&bots);

            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line");

            if buffer == "D" {
                break;
            }
        }
    }

    count
}
// WTF even is part 2
