fn recreate_df(vec: &Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    let mut count = 0;
    for &value in vec.iter() {
        if value == 0 {
            count += 1;
        } else {
            if count > 0 {
                result.push(count);
                count = 0;
            }
        }
    }
    if count > 0 {
        result.push(count);
    }
    result
}

fn find_nth_zero_sequence(vec: &Vec<u32>, n: usize) -> Option<usize> {
    let mut count = 0;
    let mut seq_count = 0;
    for (index, &value) in vec.iter().enumerate() {
        if value == 0 {
            if count == 0 {
                seq_count += 1;
            }
            if seq_count == n {
                return Some(index - count);
            }
            count += 1;
        } else {
            count = 0;
        }
    }
    None
}

pub fn get_res(path: &str) -> (u64, u64) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut vals: Vec<u32> = Vec::new();
    let mut data = true;
    let mut index = 1; // Indexes start at 1, 0 is empty space, subtract 1 after

    // Build the "disk"
    for c in file.chars() {
        let num;
        // Prevents errors
        if let Some(n) = c.to_digit(10) {
            num = n;
        } else {
            continue;
        }

        // Pushes the index or 0 to the vals vec
        if data {
            for _ in 0..num {
                vals.push(index);
            }
            index += 1;
        } else {
            for _ in 0..num {
                vals.push(0);
            }
        }
        //Flips
        data = !data;
    }

    // Save 'em for part 2
    let mut v2 = vals.clone();
    let mut df = recreate_df(&v2);

    // Part 1
    let mut i = 0;
    while i < vals.len() - 1 {
        i += 1;
        if vals[i] != 0 {
            continue;
        }

        let mut v = 0;
        while v == 0 {
            v = vals.pop().unwrap();
        }
        if i < vals.len() {
            vals[i] = v;
        } else {
            vals.push(v);
        }
    }

    for (i, el) in vals.iter().enumerate() {
        count.0 += i as u64 * (el - 1) as u64;
    }

    // Part 2

    let mut i = v2.len() - 1;
    'big: loop {
        // If we get to the first element, end
        if i <= file.clone().chars().next().unwrap().to_digit(10).unwrap() as usize {
            break 'big;
        }

        // It's an empty space, continue
        if v2[i] == 0 {
            i -= 1;
            continue 'big;
        }

        // Gets the contingent block
        let mut v = (v2[i], 0);
        while v2[i] == v.0 {
            v.1 += 1;
            i -= 1;
        }
        // v = (file index, length)

        // Finds the index of a free space big enough for the data
        let check = df.len() + 100;
        let mut free_space_index = check;

        // The fsi-th space is big enough
        for k in 0..df.len() {
            if df[k] >= v.1 {
                free_space_index = k + 1;
                break;
            }
        }

        // There's no space big enough, retry
        if free_space_index == check {
            continue 'big;
        }

        // Finds the index in v2 for the free space
        let j = find_nth_zero_sequence(&v2, free_space_index).unwrap();

        // Set the space freed as free
        for k in i + 1..=(i + v.1 as usize) {
            v2[k] = 0;
        }
        // From index j to j+v.1 put v.0
        for k in j..(j + v.1 as usize) {
            v2[k] = v.0;
        }

        df = recreate_df(&v2[0..i].to_vec());
    }

    // Removes all final zeros, useless, only for visualization
    let mut t = v2.len() - 1;
    while v2[t] == 0 {
        v2.pop();
        t -= 1;
    }

    for (k, el) in v2.iter().enumerate() {
        if *el == 0 {
            continue;
        }
        count.1 += (k as u64) * ((el - 1) as u64);
    }
    count
}
// 6382047988334 too low
// 6382582927044 too high
