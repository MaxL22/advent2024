use std::collections::HashSet;

const DIM: usize = 7;
const NUM_STEPS: usize = 12;

struct Node {
    x: usize,
    y: usize,
    dist: f32,
    heur: f32,
}

impl Node {
    fn new(x: usize, y: usize, dist: f32, heur: f32) -> Self {
        Self { x, y, dist, heur }
    }
}

fn big_pp(mat: &[[bool; DIM]; DIM]) {
    for line in mat.iter() {
        println!("{:?}", line);
    }
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let count = (0, 0);

    let mut map = [[true; DIM]; DIM];
    let mut q: Vec<Node> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    // Calculate the map
    for (i, line) in file.lines().enumerate() {
        if i == 12 {
            break;
        }

        let nums: Vec<usize> = line.split(',').map(|s| s.parse().expect("Err")).collect();
        map[nums[1]][nums[0]] = false;
    }

    big_pp(&map);

    q.push(Node::new(0, 0, 0.0, 70.0 * 70.0));

    while !q.is_empty() {}

    count
}
