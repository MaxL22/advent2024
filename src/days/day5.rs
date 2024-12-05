use std::collections::HashMap;

fn swap_values(vec: &mut Vec<i32>, val1: i32, val2: i32) {
    for item in vec.iter_mut() {
        if *item == val1 {
            *item = val2;
        } else if *item == val2 {
            *item = val1;
        }
    }
}

pub fn get_res(path: (&str, &str)) -> (i32, i32) {
    let rule_file = std::fs::read_to_string(path.0).unwrap();
    let file = std::fs::read_to_string(path.1).unwrap();
    let mut count = (0, 0);

    let mut rules: HashMap<i32, HashMap<i32, bool>> = HashMap::new();

    // Create rule map
    for line in rule_file.lines() {
        let c: Vec<i32> = line
            .split('|')
            .map(|s| s.parse::<i32>().expect("Err"))
            .collect();

        if !rules.contains_key(&c[0]) {
            rules.insert(c[0], HashMap::new());
        }

        rules.get_mut(&c[0]).unwrap().insert(c[1], true);
    }

    // Part 1
    for line in file.lines() {
        let nums = line
            .split(',')
            .map(|s| s.parse::<i32>().expect("Err"))
            .collect::<Vec<i32>>();

        let mut flag = true;

        for (j, el) in nums.iter().enumerate() {
            // Check that earlier numbers don't break the rules
            for r in nums[..j].iter() {
                if rules.get(el).unwrap().contains_key(r) {
                    flag = false;
                }
            }
        }

        // Add the number if the line it's correct
        if flag {
            count.0 += nums[nums.len() / 2];
        }
    }

    // Part 2
    for line in file.lines() {
        let mut nums: Vec<i32> = line
            .split(',')
            .map(|s| s.parse::<i32>().expect("Err 2"))
            .collect();

        let mut i = 0;
        let mut flag = false;

        while i < nums.len() {
            for j in 0..i {
                if rules.get(&nums[i]).unwrap().contains_key(&nums[j]) {
                    flag = true;
                    let v1 = nums[i];
                    let v2 = nums[j];
                    swap_values(&mut nums, v1, v2);
                    i -= 1;
                    break;
                }
            }
            i += 1;
        }

        if flag {
            count.1 += nums[nums.len() / 2];
        }
    }

    count
}
