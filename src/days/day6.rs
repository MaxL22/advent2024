use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

// Takes a step, forward or backwards, depending on the bool
fn take_a_step(direction: &Direction, coord: &mut (i32, i32), forward: bool) {
    let val: i32;
    if forward {
        val = 1
    } else {
        val = -1
    }

    match direction {
        Direction::Up => coord.0 -= val,
        Direction::Down => coord.0 += val,
        Direction::Right => coord.1 += val,
        Direction::Left => coord.1 -= val,
    }
}

// Paths the thingy, returns true if loopy
fn pathy(
    map: &mut Vec<Vec<char>>,
    mut sc: &mut (i32, i32),
    count: &mut (i32, i32),
    direction: &mut Direction,
    counter: bool,
) -> bool {
    // Tracks visited positions
    let mut hm: HashMap<(i32, i32), Direction> = HashMap::new();

    loop {
        match map[sc.0 as usize][sc.1 as usize] {
            // Go forward
            '.' => {
                if counter {
                    count.0 += 1;
                }
                map[sc.0 as usize][sc.1 as usize] = 'X';
            }
            // There's an obstacle
            '#' | 'O' => {
                // Checks if we're in a loop
                if hm.contains_key(&sc) && *hm.get(sc).unwrap() == *direction {
                    return true;
                } else {
                    hm.insert(*sc, *direction);
                }

                // Go back 1
                take_a_step(&direction, &mut sc, false);
                *direction = direction.next();
            }
            // Forward, but already seen
            _ => (),
        }

        // Next step
        take_a_step(&direction, &mut sc, true);

        // Out-of-map termination condition
        if sc.0 == map.len() as i32 || sc.1 == map[0].len() as i32 || sc.0 < 0 || sc.1 < 0 {
            return false;
        }
    }
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    // Respectively, map, coordinates and current direction
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sc: (i32, i32) = (0, 0);
    let mut direction = Direction::Up;

    // Create map
    for (i, line) in file.lines().enumerate() {
        map.push(Vec::new());

        for (j, c) in line.chars().enumerate() {
            map[i].push(c);
            if c == '^' {
                sc = (i as i32, j as i32);

                map[i][j] = 'X';
                count.0 += 1;
            }
        }
    }

    // Part 1

    // Clones them for the second part
    let og_sc = sc.clone();
    let og_map = map.clone();

    pathy(&mut map, &mut sc, &mut count, &mut direction, true);

    // Part 2
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            // For each 'X' in the first map,
            // checks whether adding an obstacle creates a loop or not
            match map[i][j] {
                'X' => {
                    // Uses new values every time
                    let mut map2 = og_map.clone();
                    let mut sc2 = og_sc.clone();
                    let mut diron = Direction::Up;
                    map2[i][j] = 'O';
                    if pathy(&mut map2, &mut sc2, &mut count, &mut diron, false) {
                        count.1 += 1;
                    }
                }
                _ => (),
            }
        }
    }

    count
}
