use regex::Regex;

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    //Part 1
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in file.lines() {
        for cap in re.captures_iter(line) {
            count.0 += &cap[1].parse::<i32>().expect("Err") * &cap[2].parse::<i32>().expect("Err");
        }
    }

    //Part 2
    let re2 = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    let mut enabled = Box::new(true);

    for line in file.lines() {
        for cap in re2.captures_iter(line) {
            match &cap[0] {
                "do()" => *enabled = true,
                "don't()" => *enabled = false,
                _ => {
                    if *enabled {
                        count.1 += &cap[2].parse::<i32>().expect("Err")
                            * &cap[3].parse::<i32>().expect("Err");
                    }
                }
            }
        }
    }

    count
}
