// On full input loops infinitely, works on test

fn build_series(to_create: &str, pieces: &Vec<String>) -> i32 {
    let mut indexes: Vec<usize> = Vec::new();
    let mut d_len = 0;
    let mut count = 0;
    indexes.push(0);

    // Iterates through all possible pieces
    // When if finds one that fits it tries to complete the word
    // Otherwise it backtracks one and tries again
    loop {
        let last_index = indexes.len() - 1;

        println!("{:?}", indexes);

        // Backtracking
        // If it hasn't found any piece for the current string
        if indexes[last_index] == pieces.len() {
            indexes.pop(); // Remove last element

            // It's over man, no can do
            if indexes.is_empty() {
                break;
            }
            d_len -= pieces[indexes[last_index - 1]].len(); // Go back to searching with the last element
            indexes[last_index - 1] += 1;
            continue;
        }

        // Found a possible piece
        if to_create[d_len..].starts_with(&pieces[indexes[last_index]]) {
            indexes.push(0);
            d_len += pieces[indexes[last_index]].len();
        } else {
            indexes[last_index] += 1;
        }

        //Count condition
        if d_len == to_create.len() {
            count += 1;

            d_len -= pieces[indexes[last_index]].len();
            indexes.pop();

            indexes[last_index] += 1;
        }
    }

    count
}

fn get_relevant_patterns(search: &str, values: &Vec<String>) -> Vec<String> {
    values
        .iter()
        .filter(|a| search.contains(*a))
        .map(String::to_owned)
        .collect()
}

pub fn get_res(path: (&str, &str)) -> (i32, i32) {
    let head = std::fs::read_to_string(path.0).unwrap();
    let file = std::fs::read_to_string(path.1).unwrap();
    let mut count = (0, 0);

    // Respectively, patterns and list of values to create
    let items: Vec<String> = head.split(", ").map(|a| a.replace('\n', "")).collect();
    let vals: Vec<&str> = file.split('\n').filter(|s| !s.is_empty()).collect();

    println!("{:?}", vals);

    for v in vals.iter() {
        let patterns = get_relevant_patterns(v, &items);
        println!("{:?}", patterns);

        // Waaaaaaay too slow, recursion is not the way
        let buildable = build_series(&v[..], &patterns);

        if buildable != 0 {
            count.0 += 1;
            count.1 += buildable;
        }
    }

    count
}
