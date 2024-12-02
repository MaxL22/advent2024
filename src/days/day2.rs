pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    for line in file.lines() {
        let c: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Err"))
            .collect();

        // Part 1

        let ord = c[0].cmp(&c[1]);
        count.0 += 1;
        let mut c1 = Box::new(true);

        for (i, el) in c[0..c.len() - 1].iter().enumerate() {
            let abs = (*el - c[i + 1]).abs();
            if el.cmp(&c[i + 1]) == ord && abs <= 3 && abs > 0 {
                continue;
            }
            count.0 -= 1;
            *c1 = false;
            break;
        }

        //Part 2
        // Need to remove the "wrong" value

        if *c1 == true {
            count.1 += 1;
            continue;
        }

        let mut c2 = Box::new(false);

        // Tries removing all values and finding a combination that works
        'f: for k in 0..c.len() {
            *c2 = true;
            let mut tmpv = c.clone();

            tmpv.remove(k);

            let ord1 = tmpv[0].cmp(&tmpv[1]);

            for (j, el) in tmpv[0..tmpv.len() - 1].iter().enumerate() {
                let abs = (*el - tmpv[j + 1]).abs();

                if el.cmp(&tmpv[j + 1]) == ord1 && abs <= 3 && abs > 0 {
                    continue;
                }
                *c2 = false;
                break;
            }

            if *c2 == true {
                break 'f;
            }
        }

        if *c2 == true {
            count.1 += 1;
        }
    }
    count
}
