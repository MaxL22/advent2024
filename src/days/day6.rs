enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        match direction {
            Direction::Up => {
                sc.0 += 1;
            }
            Direction::Down => {
                sc.0 -= 1;
            }
            Direction::Right => {
                sc.1 += 1;
            }
            Direction::Left => {
                sc.1 -= 1;
            }
        }

        map[sc.0][sc.1];

        break 'outer;
    }

    count
}
