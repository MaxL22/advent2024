use std::collections::HashMap;

fn in_map(pos: &(i32, i32), size: &i32) -> bool {
    if pos.0 < *size && pos.0 >= 0 && pos.1 < *size && pos.1 >= 0 {
        true
    } else {
        false
    }
}

fn is_free(pos: &(i32, i32), occ_map: &mut Vec<(i32, i32)>) -> bool {
    if occ_map.contains(pos) {
        false
    } else {
        occ_map.push(*pos);
        true
    }
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut map_size = 0;
    let mut cells_occ: Vec<(i32, i32)> = Vec::new();
    let mut cells_occ2: Vec<(i32, i32)> = Vec::new();

    // Creates a list of all the frequency points
    for (i, line) in file.lines().enumerate() {
        map_size += 1;
        for (j, c) in line.chars().enumerate() {
            // Skips the dots
            if c == '.' {
                continue;
            }

            // If it's a new letter
            if !map.contains_key(&c) {
                map.insert(c, Vec::new());
            }

            // Adds the coordinates to the vec
            map.get_mut(&c).unwrap().push((i as i32, j as i32));
            // Towers in the second part count as occupied nodes
            cells_occ2.push((i as i32, j as i32));
        }
    }

    // Have all coordinates, for all frequency divided
    for freq in map.clone().into_values() {
        // Each pair with each other
        for i in 0..freq.len() {
            for j in i + 1..freq.len() {
                // It's (y,x), going down
                let diff = (freq[i].0 - freq[j].0, freq[i].1 - freq[j].1);

                let pos1 = (freq[i].0 + diff.0, freq[i].1 + diff.1);
                if in_map(&pos1, &map_size) && is_free(&pos1, &mut cells_occ) {
                    count.0 += 1;
                }

                let pos2 = (freq[j].0 - diff.0, freq[j].1 - diff.1);
                if in_map(&pos2, &map_size) && is_free(&pos2, &mut cells_occ) {
                    count.0 += 1;
                }
            }
        }

        // Part 2
        for i in 0..freq.len() {
            for j in i + 1..freq.len() {
                let diff = (freq[i].0 - freq[j].0, freq[i].1 - freq[j].1);

                let mut pos1 = freq[i].clone();
                pos1 = (pos1.0 + diff.0, pos1.1 + diff.1);
                while in_map(&pos1, &map_size) {
                    if is_free(&pos1, &mut cells_occ2) {
                        count.1 += 1
                    }

                    pos1 = (pos1.0 + diff.0, pos1.1 + diff.1);
                }

                let mut pos2 = freq[j].clone();
                pos2 = (pos2.0 - diff.0, pos2.1 - diff.1);
                while in_map(&pos2, &map_size) {
                    if is_free(&pos2, &mut cells_occ2) {
                        count.1 += 1
                    }

                    pos2 = (pos2.0 - diff.0, pos2.1 - diff.1);
                }
            }
        }

        // Adds the tower nodes
        for _ in freq.iter() {
            count.1 += 1;
        }
    }

    count
}
// P2 931 too low
