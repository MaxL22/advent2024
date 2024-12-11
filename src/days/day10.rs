// Returns the position of all adjacents
fn get_adjacent_positions(matrix: &Vec<Vec<u8>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let (row, col) = position;
    let value = matrix[row][col];
    let mut adjacent_positions = Vec::new();

    let directions = vec![
        (0, 1),  // Right
        (0, -1), // Left
        (1, 0),  // Down
        (-1, 0), // Up
    ];
    for (dr, dc) in directions {
        let new_row = row.wrapping_add(dr as usize);
        let new_col = col.wrapping_add(dc as usize);

        if new_row < matrix.len() && new_col < matrix[new_row].len() {
            if matrix[new_row][new_col] == value + 1 {
                adjacent_positions.push((new_row, new_col));
            }
        }
    }

    adjacent_positions
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut map: Vec<Vec<u8>> = Vec::new();

    // Builds the map
    for (i, line) in file.lines().enumerate() {
        map.push(Vec::new());
        for c in line.chars() {
            map[i].push(c.to_digit(10).unwrap() as u8);
        }
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            // Skips it if not 0
            if map[i][j] != 0 {
                continue;
            }
            let mut mcopy = map.clone();
            let mut q: Vec<(usize, usize)> = Vec::new();

            q.push((i, j));
            while !q.is_empty() {
                let pos = q.remove(0);
                if mcopy[pos.0][pos.1] == 9 {
                    count.0 += 1;
                    mcopy[pos.0][pos.1] = 200;
                    continue;
                }

                let adj = get_adjacent_positions(&map, pos);

                for el in adj {
                    if q.contains(&el) {
                        continue;
                    }
                    q.push(el);
                }
            }

            // P2
            let mcopy = map.clone();
            let mut q: Vec<(usize, usize)> = Vec::new();

            q.push((i, j));
            while !q.is_empty() {
                let pos = q.remove(0);
                if mcopy[pos.0][pos.1] == 9 {
                    count.1 += 1;
                    continue;
                }

                q.append(&mut get_adjacent_positions(&mcopy, pos));
            }
        }
    }

    count
}
