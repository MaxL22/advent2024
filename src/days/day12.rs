use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
enum Directions {
    North,
    South,
    East,
    West,
}
const DIRS: [Directions; 4] = [
    Directions::North,
    Directions::South,
    Directions::East,
    Directions::West,
];

struct Map {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
    visited: Vec<Vec<bool>>,
}

impl Map {
    fn new(data: String) -> Self {
        let data: Vec<Vec<char>> = data.lines().map(|s| s.chars().collect()).collect();

        let height = data.len();
        let width = data[0].len();
        Self {
            height,
            width,
            data,
            visited: vec![vec![false; width]; height],
        }
    }

    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    fn adjacent(&self, c: (isize, isize)) -> Vec<(isize, isize)> {
        let mut s = vec![];
        for i in vec![(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            if self.is_within_bounds(c.0 + i.0, c.1 + i.1) {
                s.push((c.0 + i.0, c.1 + i.1));
            }
        }

        s
    }

    fn count_borders(&self, position: (usize, usize)) -> usize {
        let (row, col) = position;
        let value = self.data[row][col];
        let mut count = 0;
        let directions = vec![
            (0, 1),  // Right
            (0, -1), // Left
            (1, 0),  // Down
            (-1, 0), // Up
        ];
        for (dr, dc) in directions {
            let new_row = row.wrapping_add(dr as usize);
            let new_col = col.wrapping_add(dc as usize);
            if new_row < self.height && new_col < self.width {
                if self.data[new_row][new_col] != value {
                    count += 1;
                }
            } else {
                count += 1; // Count out of bounds as different
            }
        }
        count
    }

    fn faces(&self, c: &(usize, usize), dir: &Directions) -> bool {
        let result: bool;

        match dir {
            Directions::North => {
                if self.is_within_bounds(c.0 as isize - 1, c.1 as isize) {
                    if self.data[c.0][c.1] == self.data[c.0 - 1][c.1] {
                        result = true;
                    } else {
                        result = false;
                    }
                } else {
                    result = true;
                }
            }
            Directions::South => {
                if self.is_within_bounds((c.0 + 1) as isize, (c.1) as isize) {
                    if self.data[c.0][c.1] == self.data[c.0 + 1][c.1] {
                        result = true;
                    } else {
                        result = false;
                    }
                } else {
                    result = true;
                }
            }
            Directions::East => {
                if self.is_within_bounds((c.0) as isize, c.1 as isize - 1) {
                    if self.data[c.0][c.1] == self.data[c.0][c.1 - 1] {
                        result = true;
                    } else {
                        result = false;
                    }
                } else {
                    result = true;
                }
            }
            Directions::West => {
                if self.is_within_bounds((c.0) as isize, (c.1 + 1) as isize) {
                    if self.data[c.0][c.1] == self.data[c.0][c.1 + 1] {
                        result = true;
                    } else {
                        result = false;
                    }
                } else {
                    result = true;
                }
            }
        }

        result
    }
}

pub fn get_res(path: &str) -> (i32, i32) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0, 0);

    let mut m = Map::new(file);
    let mut borders: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut index = 0;

    let mut q = Vec::new();

    // Flood fill to get the regions
    for i in 0..m.height {
        for j in 0..m.width {
            // Already in a region
            if m.visited[i][j] {
                continue;
            }

            borders.insert(index, Vec::new());

            // Flood fill
            let o = m.data[i][j];
            let mut seen = HashSet::new();

            q.push((i as isize, j as isize));
            while let Some(t) = q.pop() {
                // Skip cases
                if seen.get(&t).is_some() {
                    continue;
                }
                seen.insert(t);

                if m.data[t.0 as usize][t.1 as usize] != o {
                    continue;
                }

                m.visited[t.0 as usize][t.1 as usize] = true;

                let adj = m.adjacent(t);
                for (a, b) in adj.clone() {
                    if m.data[a as usize][b as usize] == o {
                        q.push((a, b));
                    }
                }
                borders
                    .get_mut(&index)
                    .unwrap()
                    .push((t.0 as usize, t.1 as usize));
            }

            index += 1;
        }
    }

    // For each region, calculate borders
    for region in borders.values() {
        let mut bnumbers = 0;
        for element in region {
            bnumbers += m.count_borders(*element);
        }
        count.0 += (bnumbers * region.len()) as i32;
    }

    // Part 2
    //  For each region, find the cells facing each direction
    //  Count the sides, i.e., collapse cells having the same x (north, south) or y (east, west)

    // Doesn't work, not even conceptually

    // For each region
    for region in borders.values() {
        let mut cells_facing: HashMap<Directions, Vec<(usize, usize)>> = HashMap::new();
        for el in DIRS {
            cells_facing.insert(el, Vec::new());
        }

        // For each cell
        for cell in region.iter() {
            // Check if it faces each direction
            for dir in DIRS {
                if m.faces(&cell, &dir) {
                    // Add it to that direction if it does
                    cells_facing.get_mut(&dir).unwrap().push(*cell);
                }
            }
        }

        let mut bn = 0;
        for dir in &[Directions::North, Directions::South] {
            let mut sides: HashSet<usize> = HashSet::new();

            for val in cells_facing.get(&dir).unwrap() {
                if !sides.contains(&val.0) {
                    sides.insert(val.0);
                }
            }

            bn += sides.len();
        }

        for dir in &[Directions::East, Directions::West] {
            let mut sides: HashSet<usize> = HashSet::new();
            for val in cells_facing.get(&dir).unwrap() {
                if !sides.contains(&val.1) {
                    sides.insert(val.1);
                }
            }

            bn += sides.len();
        }

        count.1 += (bn * region.len()) as i32;
    }

    count
}
