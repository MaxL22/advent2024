fn build_series(to_create: &str, pieces: &Vec<String>) -> bool {
    let mut i = 0;

    for p in pieces.iter() {
        if to_create[i..].starts_with(p) {
            // Termination
            if p.len() + i == to_create.len() {
                return true;
            }

            i += p.len();
        }
    }

    let mut indexes: Vec<usize> = Vec::new();
    let mut last_index = 0;
    indexes.push(0);

    loop {
        // Found a possible piece
        // Add slice param
        if to_create[..].starts_with(&pieces[indexes[last_index]]) {
            indexes.push(0);
            last_index += 1;
        }
        //Termination condition

        //Backtracking

        break;
    }

    false
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

        if buildable {
            count.0 += 1;
        }
    }

    count
}
