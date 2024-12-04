const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn check_hor(j: &usize, line: &Vec<char>) -> i32 {
    let mut r = 0;

    if *j < line.len() - 3 {
        let mut f = true;
        for i in 0..4 {
            if line[*j + i] != WORD[i] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }
    if *j > 2 {
        let mut f = true;
        for i in 0..4 {
            if line[*j - i] != WORD[i] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }

    r
}

fn check_vert(i: &usize, j: &usize, mat: &Vec<Vec<char>>) -> i32 {
    let mut r = 0;

    if *i < mat.len() - 3 {
        let mut f = true;
        for k in 0..4 {
            if mat[*i + k][*j] != WORD[k] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }
    if *i > 2 {
        let mut f = true;
        for k in 0..4 {
            if mat[*i - k][*j] != WORD[k] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }

    r
}

fn check_diag(i: &usize, j: &usize, mat: &Vec<Vec<char>>) -> i32 {
    let mut r = 0;

    if *i > 2 && *j > 2 {
        let mut f = true;
        for k in 0..4 {
            if mat[*i - k][*j - k] != WORD[k] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }

    if *i < mat.len() - 3 && *j > 2 {
        let mut f = true;
        for k in 0..4 {
            if mat[*i + k][*j - k] != WORD[k] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }

    if *i > 2 && *j < mat[0].len() - 3 {
        let mut f = true;
        for k in 0..4 {
            if mat[*i - k][*j + k] != WORD[k] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }

    if *i < mat.len() - 3 && *j < mat[0].len() - 3 {
        let mut f = true;
        for k in 0..4 {
            if mat[*i + k][*j + k] != WORD[k] {
                f = false;
                break;
            }
        }
        if f {
            r += 1;
        }
    }
    r
}

fn check_p2(i: &usize, j: &usize, mat: &Vec<Vec<char>>) -> bool {
    let mut f = false;

    if mat[*i][*j] != 'A' {
        return false;
    }

    match mat[*i - 1][*j - 1] {
        'M' => match mat[*i - 1][*j + 1] {
            'M' => {
                if mat[*i + 1][*j - 1] == 'S' && mat[*i + 1][*j + 1] == 'S' {
                    f = true;
                }
            }
            'S' => {
                if mat[*i + 1][*j - 1] == 'M' && mat[*i + 1][*j + 1] == 'S' {
                    f = true;
                }
            }
            _ => f = false,
        },
        'S' => match mat[*i - 1][*j + 1] {
            'M' => {
                if mat[*i + 1][*j - 1] == 'S' && mat[*i + 1][*j + 1] == 'M' {
                    f = true;
                }
            }
            'S' => {
                if mat[*i + 1][*j - 1] == 'M' && mat[*i + 1][*j + 1] == 'M' {
                    f = true;
                }
            }
            _ => f = false,
        },
        _ => f = false,
    }

    f
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    //Stored as matrix

    let mut mat: Vec<Vec<char>> = Vec::new();

    for _ in 0..file.find('\n').unwrap() {
        mat.push(Vec::new());
    }

    //Part 1

    //Makes the input a matrix
    let mut index = 0;
    for line in file.lines() {
        for c in line.chars() {
            mat[index].push(c);
        }
        index += 1;
    }

    //Checks occurrences
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let res = check_hor(&j, &mat[i]);
            count.0 += res;
            let res = check_vert(&i, &j, &mat);
            count.0 += res;
            let res = check_diag(&i, &j, &mat);
            count.0 += res;
        }
    }

    //Part 2
    for i in 1..mat.len() - 1 {
        for j in 1..mat[i].len() - 1 {
            let res = check_p2(&i, &j, &mat);
            if res {
                count.1 += 1;
            }
        }
    }

    count
}
