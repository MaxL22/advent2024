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

    // Misses a case, e.g., (3,3) of the test, other side is in region and is not counted as angle
    fn corners(&self, position: &(isize, isize), region: &Vec<(usize, usize)>) -> i32 {
        let mut c = 0;

        let v1 = vec![(1, 0), (0, 1), (1, 1)];
        let v2 = vec![(1, 0), (0, -1), (1, -1)];

        let mut count = (0, 0);
        for val in v1 {
            let np = (position.0 + val.0, position.1 + val.1);
            let np2 = (position.0 - val.0, position.1 - val.1);

            if !self.is_within_bounds(np.0, np.1)
                || !region.contains(&(np.0 as usize, np.1 as usize))
            {
                count.0 += 1;
            }
            if !self.is_within_bounds(np2.0, np2.1)
                || !region.contains(&(np2.0 as usize, np2.1 as usize))
            {
                count.1 += 1;
            }
        }

        if count.0 == 3 {
            c += 1;
        }
        if count.1 == 3 {
            c += 1;
        }
        if count.0 == 1 {
            if !self.is_within_bounds(position.0 + 1, position.1 + 1)
                || !region.contains(&(position.0 as usize + 1, position.1 as usize + 1))
            {
                c += 1;
            }
        }
        if count.1 == 1 {
            if !self.is_within_bounds(position.0 - 1, position.1 - 1)
                || !region.contains(&(position.0 as usize - 1, position.1 as usize - 1))
            {
                c += 1;
            }
        }

        let mut count = (0, 0);
        for val in v2 {
            let np = (position.0 + val.0, position.1 + val.1);
            let np2 = (position.0 - val.0, position.1 - val.1);

            if !self.is_within_bounds(np.0, np.1)
                || !region.contains(&(np.0 as usize, np.1 as usize))
            {
                count.0 += 1;
            }
            if !self.is_within_bounds(np2.0, np2.1)
                || !region.contains(&(np2.0 as usize, np2.1 as usize))
            {
                count.1 += 1;
            }
        }

        if count.0 == 3 {
            c += 1;
        }
        if count.1 == 3 {
            c += 1;
        }
        if count.0 == 1 {
            if !self.is_within_bounds(position.0 + 1, position.1 - 1)
                || !region.contains(&(position.0 as usize + 1, position.1 as usize - 1))
            {
                c += 1;
            }
        }
        if count.1 == 1 {
            if !self.is_within_bounds(position.0 - 1, position.1 + 1)
                || !region.contains(&(position.0 as usize - 1, position.1 as usize + 1))
            {
                c += 1;
            }
        }

        println!("{:?}, {:?}", position, c);

        c
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
    // Count the corners, check if each cell is a corner

    // For each region
    for region in borders.values() {
        // For each region
        let mut corners = 0;
        for cell in region.iter() {
            corners += m.corners(&(cell.0 as isize, cell.1 as isize), &region);
        }
        println!("{corners}");
        count.1 += corners * region.len() as i32;
    }

    count
}
// 928092 too low
