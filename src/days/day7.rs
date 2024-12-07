// Generates all possible permutations of size n for + (0) and * (1)
fn generate_binary_permutations(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let total = 1 << n; // 2^n permutations

    for i in 0..total {
        let mut binary_string = String::new();
        for j in (0..n).rev() {
            if (i & (1 << j)) != 0 {
                binary_string.push('*');
            } else {
                binary_string.push('+');
            }
        }
        result.push(binary_string);
    }

    result
}

fn generate_ternary_permutations(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let values = ['*', '+', '|'];
    let total = 3_usize.pow(n as u32);
    for i in 0..total {
        let mut ternary_string = String::new();
        let mut num = i;
        for _ in 0..n {
            let index = num % 3;
            ternary_string.push(values[index]);
            num /= 3;
        }
        result.push(ternary_string.chars().rev().collect::<String>());
    }
    result
}

fn concatenate_numbers_i64(a: i64, b: i64) -> i64 {
    let concatenated_string = format!("{}{}", a, b);
    concatenated_string.parse::<i64>().unwrap()
}

pub fn get_res(path: &str) -> (i64, i64) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    for line in file.lines() {
        let tmp: Vec<&str> = line.split(":").collect();

        // Target num, sequence
        let goal = tmp[0].parse::<i64>().expect("Err");
        let nums: Vec<i64> = tmp[1]
            .split_whitespace()
            .map(|s| s.parse::<i64>().expect("Err"))
            .collect();

        // Part 1
        let permutations = generate_binary_permutations(nums.len() - 1);

        // For each possible permutation
        for el in permutations.iter() {
            let mut val;

            if el.chars().nth(0).unwrap() == '+' {
                val = nums[0] + nums[1];
            } else {
                val = nums[0] * nums[1];
            }

            // Calculates each sum/prod
            for (i, c) in el[1..el.len()].chars().enumerate() {
                match c {
                    '*' => val *= nums[i + 2],
                    '+' => val += nums[i + 2],
                    _ => (),
                }
            }
            if val == goal {
                count.0 += val;
                break;
            }
        }

        // Part 2
        let permutations = generate_ternary_permutations(nums.len() - 1);

        for el in permutations.iter() {
            let mut val;

            if el.chars().nth(0).unwrap() == '+' {
                val = nums[0] + nums[1];
            } else if el.chars().nth(0).unwrap() == '*' {
                val = nums[0] * nums[1];
            } else {
                val = concatenate_numbers_i64(nums[0], nums[1]);
            }

            // Calculates each sum/prod
            for (i, c) in el[1..el.len()].chars().enumerate() {
                match c {
                    '*' => val *= nums[i + 2],
                    '+' => val += nums[i + 2],
                    '|' => val = concatenate_numbers_i64(val, nums[i + 2]),
                    _ => (),
                }
            }
            if val == goal {
                count.1 += val;
                break;
            }
        }
    }

    count
}
