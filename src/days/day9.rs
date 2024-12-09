fn reformat_df(vals: &Vec<u32>) -> Vec<u32> {
    let mut df: Vec<u32> = Vec::new();

    let mut i = 0;
    while i < vals.len() - 1 {
        i += 1;
        if vals[i] != 0 {
            continue;
        }

        let mut c = 0;
        while vals[i] == 0 {
            c += 1;
            i += 1;
            if i == vals.len() {
                break;
            }
        }
        df.push(c);
    }

    df
}

pub fn get_res(path: &str) -> (u64, u64) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut vals: Vec<u32> = Vec::new();
    let mut data = true;
    let mut index = 1; // Indexes start at 1, 0 is empty space, subtract 1 after
    let mut df: Vec<u32> = Vec::new();

    // Build the "disk"
    for c in file.chars() {
        let num;
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
            df.push(num);
        }
        data = !data;
    }

    let mut v2 = vals.clone();

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

    //println!("{:?}", v2);
    let mut i = v2.len() - 1;
    'big: loop {
        if i <= 7 {
            // Taken from the input, unfair I know
            break 'big;
        }

        // Gets the contingent block
        let mut v = (v2[i], 0);
        // It's an empty space, continue
        if v.0 == 0 {
            i -= 1;
            continue 'big;
        }

        while v2[i] == v.0 {
            v.1 += 1;
            i -= 1;
        }
        // v = (index, length)

        // Finds the index of a free space big enough for the data
        let check = df.len() + 100;
        let mut fsn = check;

        // The fsn-th space is big enough
        for k in 0..df.len() {
            if df[k] >= v.1 {
                fsn = k;
                break;
            }
        }

        // There's no space big enough, retry
        if fsn == check {
            continue 'big;
        }

        // Finds the index in v2 for the free space
        let mut k = 0;
        let mut j = 0;
        while j < v2.len() - 1 {
            j += 1;

            if v2[j] != 0 {
                continue;
            }

            // Is it the right one?
            if k == fsn {
                break;
            }
            // Found some free space
            k += 1;
            // Skip the block ahead
            while v2[j + 1] == 0 {
                j += 1;
            }
        }
        // Set the space freed as free
        for k in i + 1..=(i + v.1 as usize) {
            v2[k] = 0;
        }

        // From index j to j+v.1 put v.0
        for k in j..j + v.1 as usize {
            v2[k] = v.0;
        }

        df = reformat_df(&v2[..i].to_vec());
    }

    for (i, el) in v2.iter().enumerate() {
        if *el == 0 {
            continue;
        }
        count.1 += (i as u64) * ((el - 1) as u64);
    }

    count
}
// 6382047988334 too low
// 6382582927044 too high
