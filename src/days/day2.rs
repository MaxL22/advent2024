pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    for line in file.lines() {
        let mut c: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Err"))
            .collect();

        // Part 1
        let ord = c[0].cmp(&c[1]);

        count.0 += 1;
        for (i, el) in c[0..c.len() - 1].iter().enumerate() {
            let abs = (*el - c[i + 1]).abs();
            if el.cmp(&c[i + 1]) == ord && abs <= 3 && abs > 0 {
                continue;
            }
            count.0 -= 1;
            break;
        }

        //Part 2
        // Need to remove the "wrong" value
        count.1 += 1;
        let mut failsafe = true;

        let mut i = 1;
        'w: while i < c.len() {
            let o1 = c[1].cmp(&c[0]);
            let abs = (c[i] - c[i - 1]).abs();
            println!("{:?} {:?} {abs} {failsafe}", c[i].cmp(&c[i - 1]), o1);

            if c[i].cmp(&c[i - 1]) != o1 || abs > 3 || abs == 0 {
                if failsafe == true {
                    failsafe = false;
                    c.remove(i);
                    continue;
                }
                count.1 -= 1;
                break 'w;
            }
            i += 1;
        }
    }
    count
}
