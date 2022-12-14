use crate::utils::*;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Matter {
    Air,
    Rock,
    Sand,
}

struct Wall {
    offset: isize,
    grid: Vec<Vec<Matter>>,
}

impl Wall {
    fn new(input: &str) -> Self {
        use Matter::*;

        let max_y = Self::scan_height(input);
        let height = 1 + max_y + 2;
        let (offset, width) = (500 - height, 2 * height);

        let mut grid = vec![vec![Air; width as usize]; (height - 1) as usize];
        grid.push(vec![Rock; width as usize]);

        let mut w = Self { offset, grid };

        for line in file_lines(input) {
            let mut coords = line
                .split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(x, y)| (must_parse::<isize>(x), must_parse::<isize>(y)));
            let mut start = coords.next().unwrap();
            for end in coords {
                let d = (
                    (end.0 - start.0).clamp(-1, 1),
                    (end.1 - start.1).clamp(-1, 1),
                );
                while start != end {
                    w[start] = Rock;
                    start.0 += d.0;
                    start.1 += d.1;
                }
            }
            w[start] = Rock;
        }

        w
    }

    fn scan_height(input: &str) -> isize {
        file_lines(input)
            .flat_map(|l| {
                let v: Vec<isize> = l
                    .split(" -> ")
                    .map(|s| s.split_once(',').unwrap().1)
                    .map(must_parse::<isize>)
                    .collect();
                v
            })
            .max()
            .unwrap()
    }

    fn drop_to_bottom(&mut self) -> usize {
        let mut ct = 0;
        let target = (self.grid.len() - 2) as isize;
        let mut prev = vec![(500, 0)];
        while self.drop_sand(&mut prev).1 < target {
            ct += 1;
        }
        ct
    }

    fn drop_to_top(&mut self) -> usize {
        let mut ct = 0;
        let mut prev = vec![(500, 0)];
        while self.drop_sand(&mut prev) != (500, 0) {
            ct += 1;
        }
        ct + 1
    }

    fn drop_sand(&mut self, prev: &mut Vec<(isize, isize)>) -> (isize, isize) {
        while let Some(&pos) = prev.last() {
            let mut test = (pos.0, pos.1 + 1);
            if self[test] == Matter::Air {
                prev.push(test);
                continue;
            }

            test = (pos.0 - 1, pos.1 + 1);
            if self[test] == Matter::Air {
                prev.push(test);
                continue;
            }

            test = (pos.0 + 1, pos.1 + 1);
            if self[test] == Matter::Air {
                prev.push(test);
                continue;
            }

            prev.pop();
            self[pos] = Matter::Sand;
            return pos;
        }

        unreachable!()
    }
}

impl Debug for Wall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Matter::*;
        for (r, row) in self.grid.iter().enumerate() {
            let s: String = row
                .iter()
                .enumerate()
                .map(|(c, tile)| match &tile {
                    _ if (c + self.offset as usize, r) == (500, 0) => '+',
                    Air => ' ',
                    Rock => '#',
                    Sand => 'o',
                })
                .collect();
            writeln!(f, "{s}")?
        }
        Ok(())
    }
}

impl Index<(isize, isize)> for Wall {
    type Output = Matter;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        &self.grid[y as usize][(x - self.offset) as usize]
    }
}

impl IndexMut<(isize, isize)> for Wall {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        &mut self.grid[y as usize][(x - self.offset) as usize]
    }
}

fn solve_1(input: &str) -> usize {
    Wall::new(input).drop_to_bottom()
}

fn solve_2(input: &str) -> usize {
    Wall::new(input).drop_to_top()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec14/example_1.txt";
        assert_eq!(24, solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec14/input_1.txt";
        assert_eq!(683, solve_1(input));
    }
    #[test]
    fn example_2() {
        let input = "src/dec14/example_1.txt";
        assert_eq!(93, solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec14/input_1.txt";
        assert_eq!(28821, solve_2(input));
    }
}
