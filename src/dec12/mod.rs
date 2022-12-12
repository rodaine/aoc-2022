use crate::utils::*;
use std::collections::{BTreeSet, HashMap};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Idx(usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct DistIdx(usize, Idx);

impl DistIdx {
    fn new(idx: Idx, dist: usize) -> Self {
        Self(dist, idx)
    }
}

struct Square {
    elevation: u32,
    neighbors: Vec<Idx>,
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        let elevation = match value {
            'S' => 'a' as u32,
            'E' => 'z' as u32,
            _ => value as u32,
        };
        Self {
            elevation,
            neighbors: Vec::new(),
        }
    }
}

struct Mountain {
    squares: Vec<Vec<Square>>,
    start: Idx,
    end: Idx,
    maybe_starts: Vec<Idx>,
}

const DIR_DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Mountain {
    fn all_vertices(&self) -> impl Iterator<Item = Idx> {
        let rrng = 0..self.squares.len();
        let crng = 0..self.squares[0].len();
        rrng.flat_map(move |r| crng.clone().map(move |c| Idx(r, c)))
    }

    fn neighbors(&self, pos: Idx) -> impl Iterator<Item = Idx> {
        let (r, c) = (pos.0 as isize, pos.1 as isize);
        let rrng = 0..self.squares.len() as isize;
        let crng = 0..self.squares[0].len() as isize;

        DIR_DELTAS
            .iter()
            .map(move |(dr, dc)| (r + dr, c + dc))
            .filter(move |(r, c)| rrng.contains(r) && crng.contains(c))
            .map(|(r, c)| Idx(r as usize, c as usize))
    }

    fn walk(&self) -> (HashMap<Idx, usize>, HashMap<Idx, Idx>) {
        let mut prev: HashMap<Idx, Idx> = HashMap::new();
        let mut dist: HashMap<Idx, usize> = HashMap::new();
        let mut queue: BTreeSet<DistIdx> = BTreeSet::new();

        dist.insert(self.end, 0);
        queue.insert(DistIdx::new(self.end, 0));

        while let Some(DistIdx(_, idx)) = queue.pop_first() {
            let alt = dist.get(&idx).copied().map(|i| i + 1).unwrap_or(usize::MAX);
            for v in &self[idx].neighbors {
                let dv = dist.get(v).copied().unwrap_or(usize::MAX);
                if alt < dv {
                    dist.insert(*v, alt);
                    prev.insert(*v, idx);
                    queue.remove(&DistIdx::new(*v, dv));
                    queue.insert(DistIdx::new(*v, alt));
                }
            }
        }

        (dist, prev)
    }

    fn shortest_path(&self, start: Idx) -> usize {
        let (dist, _) = self.walk();
        dist[&start]
    }

    fn shortest_start(&self) -> usize {
        let (dist, _) = self.walk();

        self.maybe_starts
            .iter()
            .filter_map(|idx| dist.get(idx).copied())
            .min()
            .unwrap()
    }
}

impl Index<Idx> for Mountain {
    type Output = Square;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.squares[index.0][index.1]
    }
}

impl IndexMut<Idx> for Mountain {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.squares[index.0][index.1]
    }
}

impl FromIterator<String> for Mountain {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut squares = Vec::new();
        let mut start = Idx(0, 0);
        let mut end = Idx(0, 0);
        let mut maybe_starts = Vec::new();

        for (r, line) in iter.into_iter().enumerate() {
            let mut row = Vec::new();
            for (c, ch) in line.chars().enumerate() {
                let sq = Square::from(ch);
                match ch {
                    'S' => {
                        start = Idx(r, c);
                        maybe_starts.push(Idx(r, c));
                    }
                    'E' => end = Idx(r, c),
                    'a' => maybe_starts.push(Idx(r, c)),
                    _ => {}
                };
                row.push(sq);
            }
            squares.push(row);
        }

        let mut mountain = Self {
            squares,
            start,
            end,
            maybe_starts,
        };

        for idx in mountain.all_vertices() {
            let elev = mountain[idx].elevation;
            for neighbor in mountain.neighbors(idx) {
                let nelev = mountain[neighbor].elevation;
                if nelev >= elev - 1 {
                    mountain[idx].neighbors.push(neighbor)
                }
            }
        }

        mountain
    }
}

fn solve_1(input: &str) -> usize {
    let mountain = Mountain::from_iter(file_lines(input));
    mountain.shortest_path(mountain.start)
}

fn solve_2(input: &str) -> usize {
    let mountain = Mountain::from_iter(file_lines(input));
    mountain.shortest_start()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec12/example_1.txt";
        assert_eq!(31, solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec12/input_1.txt";
        assert_eq!(447, solve_1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec12/example_1.txt";
        assert_eq!(29, solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec12/input_1.txt";
        assert_eq!(446, solve_2(input));
    }
}
