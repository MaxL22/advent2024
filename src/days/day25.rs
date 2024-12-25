pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    // Locks and keys, respectively
    let mut vals: Vec<Vec<Vec<i32>>> = Vec::new();
    vals.push(Vec::new());
    vals.push(Vec::new());

    let mut c = 0;
    let mut is_lock = 0;

    for line in file.lines() {
        c = (c + 1) % 8;
        match c {
            // Empty line
            0 => {
                continue;
            }
            // Start of lock/key
            1 => {
                if line == "#####" {
                    is_lock = 0;
                } else {
                    is_lock = 1;
                }

                vals[is_lock].push(vec![0, 0, 0, 0, 0]);
            }
            // Inside a lock/key
            _ => {
                let index = vals[is_lock].len() - 1;
                for (i, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        vals[is_lock][index][i] += 1;
                    }
                }
            }
        }
    }

    // Keys were 1 over, easiest way
    for el in vals[1].iter_mut() {
        el.iter_mut().for_each(|x| *x -= 1);
    }

    // Count pairings
    for key in vals[1].iter() {
        for lock in vals[0].iter() {
            count.0 += 1;
            for i in 0..lock.len() {
                if key[i] + lock[i] > 5 {
                    count.0 -= 1;
                    break;
                }
            }
        }
    }

    count
}
