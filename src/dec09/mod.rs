use crate::utils::*;
use std::collections::HashSet;
use std::iter::repeat;

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        use Dir::*;
        match value {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => unreachable!(),
        }
    }
}

impl From<Dir> for (isize, isize) {
    fn from(value: Dir) -> Self {
        use Dir::*;
        match value {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

#[derive(Default)]
struct Knot {
    hx: isize,
    hy: isize,
    tx: isize,
    ty: isize,
}

impl Knot {
    fn step_towards(&mut self, dir: Dir) {
        let (dx, dy): (isize, isize) = dir.into();
        self.step_to(self.hx + dx, self.hy + dy);
    }

    fn step_to(&mut self, x: isize, y: isize) {
        self.hx = x;
        self.hy = y;

        let (dx, dy) = (self.hx - self.tx, self.hy - self.ty);
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }

        self.tx += dx.clamp(-1, 1);
        self.ty += dy.clamp(-1, 1);
    }
}

struct Chain {
    knots: Vec<Knot>,
    history: HashSet<(isize, isize)>,
}

impl Chain {
    fn new(n: usize) -> Self {
        let knots = (0..n).map(|_| Knot::default()).collect();
        let mut history = HashSet::new();
        history.insert((0, 0));
        Self { knots, history }
    }

    fn step(&mut self, d: Dir) {
        self.knots[0].step_towards(d);
        let (mut x, mut y) = (self.knots[0].tx, self.knots[0].ty);

        if self.knots.len() > 1 {
            for knot in &mut self.knots[1..] {
                knot.step_to(x, y);
                (x, y) = (knot.tx, knot.ty)
            }
        }

        self.history.insert((x, y));
    }

    fn read_path<I: Iterator<Item = String>>(&mut self, iter: I) {
        iter.into_iter()
            .flat_map(|s: String| {
                let (d, n) = s.split_once(' ').unwrap();
                let dir = Dir::from(d);
                let ct = must_parse(n);
                repeat(dir).take(ct)
            })
            .for_each(|d| self.step(d));
    }
}

fn solve_1(input: &str) -> usize {
    let mut chain = Chain::new(1);
    chain.read_path(file_lines(input));
    chain.history.len()
}

fn solve_2(input: &str) -> usize {
    let mut chain = Chain::new(9);
    chain.read_path(file_lines(input));
    chain.history.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec09/example_1.txt";
        assert_eq!(13, solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec09/input_1.txt";
        assert_eq!(6269, solve_1(input));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, solve_2("src/dec09/example_1.txt"));
        assert_eq!(36, solve_2("src/dec09/example_2.txt"));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec09/input_1.txt";
        assert_eq!(2557, solve_2(input));
    }
}
