fn build_series(to_create: &str, pieces: &Vec<String>) -> bool {
    let mut built = false;

    for p in pieces.iter() {
        if to_create.starts_with(p) {
            // Termination
            if to_create.len() - p.len() == 0 {
                return true;
            }

            built = build_series(&to_create[p.len()..], pieces);
        }
    }

    built
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
