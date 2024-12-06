enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

fn dir_update(direction: Direction, coord: &mut (i32, i32), forward: bool) {
    let val;

    if forward {
        val = 1
    } else {
        val = -1
    }

    match direction {
        Direction::Up => coord.0 += val,
        Direction::Down => coord.0 -= val,
        Direction::Right => coord.1 += val,
        Direction::Left => coord.1 -= val,
    }
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sc: (usize, usize) = (0, 0);
    let mut direction = Direction::Up;

    // Create map
    for (i, line) in file.lines().enumerate() {
        map.push(Vec::new());

        for c in line.chars() {
            map[i].push(c);
            if c == '^' {
                sc = (i, map[i].len() - 1);
                let v = map.len() - 1;
                map[i][v] = 'X';
                count.0 += 1;
            }
        }
    }

    //Part 1
    'outer: loop {
        match map[sc.0][sc.1] {
            //Go forward
            '.' => {
                count.0 += 1;
                map[sc.0][sc.1] = 'X';
            }
            //There's an obstacle
            '#' => {
                direction = (direction as usize + 1) % 4;
                //Go back 1
                dir_update(direction, &mut sc, false);
            }
            // Forward, but already seen
            'X' => {}
        }

        //Next step
        dir_update(direction, &mut sc, a);

        break 'outer;
    }

    count
}
