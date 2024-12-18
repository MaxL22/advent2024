use std::collections::HashSet;

// Change these to use the test
const DIM: usize = 71;
const NUM_STEPS: usize = 1024;

// Keeps the coordinates, distance from the start and heuristic to the end
#[derive(Clone, Copy, Debug)]
struct Node {
    x: usize,
    y: usize,
    dist: i32,
    heur: f32, // Also contains the distance
}

impl Node {
    fn new(x: usize, y: usize, dist: i32, heur: f32) -> Self {
        Self { x, y, dist, heur }
    }

    // Returns the nodes adjacent to itself
    fn get_adjacent_indices(&self) -> Vec<(usize, usize)> {
        let mut adjacent_indices = Vec::new();
        let directions = [
            (0, 1),
            (1, 0),  // down
            (0, -1), // left
            (-1, 0), // up
        ];

        for direction in &directions {
            let new_x = self.x as isize + direction.0;
            let new_y = self.y as isize + direction.1;

            if new_x >= 0 && new_x < DIM as isize && new_y >= 0 && new_y < DIM as isize {
                adjacent_indices.push((new_x as usize, new_y as usize));
            }
        }

        adjacent_indices
    }
}

// Gets the file, parses it and drops the first num_steps bytes
fn get_path(file: String, num_steps: usize) -> Option<Node> {
    let mut map = [[true; DIM]; DIM];
    let mut q: Vec<(Node, i32)> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    // Calculate the map
    for (i, line) in file.lines().enumerate() {
        if i == num_steps {
            break;
        }

        let nums: Vec<usize> = line.split(',').map(|s| s.parse().expect("Err")).collect();
        map[nums[1]][nums[0]] = false;
    }

    // Start the q
    q.push((Node::new(0, 0, 0, 0.0 + 2.0 * DIM as f32 - 2.0), 0));
    let mut final_node: Option<Node> = None;
    let mut path: Vec<Node> = Vec::new();

    // It's just A*
    while !q.is_empty() {
        // Get the node
        let val = q.pop().unwrap();
        path.push(val.0);

        // Add node to seen
        seen.insert((val.0.x, val.0.y));

        // If last node, get and get out
        if val.0.x == DIM - 1 && val.0.y == DIM - 1 {
            final_node = Some(val.0.clone());
            break;
        }

        // For each adjacent coordinate
        for ind in val.0.get_adjacent_indices() {
            if seen.contains(&(ind.0, ind.1)) || map[ind.1][ind.0] == false {
                continue;
            }

            // Set node as seen
            seen.insert((ind.0, ind.1));
            // Add new node into q
            q.push((
                Node::new(
                    ind.0,
                    ind.1,
                    val.1 + 1,
                    val.1 as f32 + 1.0 + 2.0 * DIM as f32 - 2.0 - ind.0 as f32 - ind.1 as f32,
                ),
                val.1 + 1,
            ));
        }

        // Sort the q to get the min value, could be a heap
        q.sort_by(|a, b| b.0.heur.total_cmp(&a.0.heur));
    }

    // Returns an Option to the final node
    final_node
}

pub fn get_res(path: &str) -> (i32, (usize, usize)) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, (0, 0));

    // Part 1
    count.0 += get_path(file.clone(), NUM_STEPS)
        .unwrap_or(Node::new(0, 0, 0, 0.0))
        .dist;

    // Check the path, one fallen byte at a time
    let mut row_n: usize = 0;
    for i in NUM_STEPS..file.clone().lines().count() {
        let node = get_path(file.clone(), i);

        if let Some(_) = node {
        } else {
            // If there's no path, end the loop
            row_n = i - 1;
            break;
        }
    }

    // Parse the numbers
    //    (I could've left it as a string)
    let v: Vec<usize> = file
        .lines()
        .nth(row_n)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().expect("Err"))
        .collect();

    // But this way it's dumber
    count.1 .0 = v[0];
    count.1 .1 = v[1];

    count
}
// Takes a while, ~18s to end both parts
