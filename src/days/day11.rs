use std::collections::HashMap;

fn has_even_number_of_digits(num: &i64) -> bool {
    let num_str = num.to_string();
    let digit_count = num_str.len();
    digit_count % 2 == 0
}

fn split_number(num: &i64) -> (i64, i64) {
    let num_str = num.abs().to_string();
    let len = num_str.len();
    let mid = len / 2;
    let left_half: i64 = num_str[..mid].parse().unwrap();
    let right_half: i64 = num_str[mid..].parse().unwrap();
    if *num < 0 {
        (-left_half, right_half)
    } else {
        (left_half, right_half)
    }
}

fn blink(mut stones: Vec<i64>, num: usize) -> Vec<i64> {
    let mut i = 0;
    while i < num {
        let mut j = 0;
        for _ in 0..stones.len() {
            if stones[j] == 0 {
                stones[j] = 1;
            } else if has_even_number_of_digits(&stones[j]) {
                let vals = split_number(&stones[j]);
                stones[j] = vals.0;
                j += 1;
                stones.insert(j, vals.1);
            } else {
                stones[j] = stones[j] * 2024;
            }
            j += 1;
        }

        i += 1;
    }

    stones
}

fn to_counter(values: Vec<i64>) -> HashMap<i64, u128> {
    let mut counter: HashMap<i64, u128> = HashMap::new();

    for el in values {
        if !counter.contains_key(&el) {
            counter.insert(el, 0);
        }
        *counter.get_mut(&el).unwrap() += 1;
    }
    counter
}

fn count_blink(stone_count: HashMap<i64, u128>) -> HashMap<i64, u128> {
    let mut new_count: HashMap<i64, u128> = HashMap::new();

    for el in stone_count.keys() {
        // The val is 0, becomes 1
        if *el == 0 {
            if !new_count.contains_key(&1) {
                new_count.insert(1, 0);
            }
            *new_count.get_mut(&1).unwrap() += *stone_count.get(el).unwrap_or(&1);
        // If the num has even number of digits
        } else if has_even_number_of_digits(el) {
            // Split it and add to each part
            let vals = split_number(el);

            // First half
            if !new_count.contains_key(&vals.0) {
                new_count.insert(vals.0, 0);
            }
            *new_count.get_mut(&vals.0).unwrap() += *stone_count.get(el).unwrap_or(&1);

            // Secodn half
            if !new_count.contains_key(&vals.1) {
                new_count.insert(vals.1, 0);
            }
            *new_count.get_mut(&vals.1).unwrap() += *stone_count.get(el).unwrap_or(&1);
        } else {
            // Every other number
            if !new_count.contains_key(&(*el * 2024)) {
                new_count.insert(*el * 2024, 0);
            }
            *new_count.get_mut(&(el * 2024)).unwrap() += *stone_count.get(el).unwrap_or(&1);
        }
    }

    new_count
}

pub fn get_res(path: &str) -> (i32, u128) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut stones: Vec<i64> = file
        .split_whitespace()
        .map(|s| s.parse().expect("Err"))
        .collect();

    let s = stones.clone();

    // Part 1

    stones = blink(stones, 25);

    count.0 = stones.len() as i32;

    // Part 2

    let mut stone_count: HashMap<i64, u128> = to_counter(s);

    for _ in 0..75 {
        stone_count = count_blink(stone_count);
    }

    for el in stone_count.into_values() {
        count.1 += el;
    }

    count
}
