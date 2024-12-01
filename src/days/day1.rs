use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_res(file_path: &str) -> (i32, i32) {
    //Open file
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    //Init values and result tuple
    let mut values: Vec<Vec<i32>> = Vec::new();
    let mut count = (0, 0);

    for _ in 0..2 {
        values.push(Vec::new());
    }

    //Gets all the values
    for line in reader.lines() {
        let l: String = line.unwrap().parse::<String>().unwrap();
        let tval: Vec<i32> = l
            .split_whitespace()
            .map(|c| c.parse::<i32>().expect("Err parsing"))
            .collect();

        for i in 0..2 {
            values[i].push(tval[i]);
        }
    }

    for i in 0..2 {
        values[i].sort();
    }

    //Part 1
    for i in 0..values[0].len() {
        let d = values[0][i] - values[1][i];
        count.0 += i32::abs(d);
    }

    //Part 2
    let mut index2 = 0;

    // Kinda double pointer, since the list is sorted
    for element in values[0].iter() {
        let mut c = 0;

        while *element >= values[1][index2] {
            if *element == values[1][index2] {
                c += 1;
            }
            index2 += 1;
        }

        count.1 += element * c;
    }

    count
}
