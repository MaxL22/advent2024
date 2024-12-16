#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub fn get_res(path: (&str, &str)) -> (i32, i32) {
    let file = std::fs::read_to_string(path.1).unwrap();
    let mfile = std::fs::read_to_string(path.0).unwrap();
    let mut count = (0, 0);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<Direction> = Vec::new();

    for (i, line) in mfile.lines().enumerate() {
        map.push(Vec::new());
        for c in line.chars() {
            map[i].push(c);
        }
    }

    for c in file.chars() {
        match c {
            '<' => moves.push(Direction::Left),
            '>' => moves.push(Direction::Right),
            '^' => moves.push(Direction::Up),
            'v' => moves.push(Direction::Down),
            _ => {}
        }
    }

    for line in map.iter() {
        println!("{:?}", line);
    }

    println!("");

    println!("{:?}", moves);

    count
}
