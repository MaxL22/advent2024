use std::collections::{HashMap, HashSet};

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

    count
}
