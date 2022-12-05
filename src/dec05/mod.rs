use crate::utils::*;
use std::vec::Vec;

#[derive(Default)]
struct Stack(Vec<Vec<char>>);

impl Stack {
    fn new<I: Iterator<Item = String>>(iter: &mut I) -> Self {
        let mut s = Self::default();
        for line in iter {
            if line.is_empty() {
                break;
            }
            s.add_line(line)
        }

        for v in &mut s.0 {
            v.reverse()
        }
        s
    }

    fn add_line(&mut self, line: String) {
        let mut chars = line.chars();
        let mut idx = 0;

        loop {
            idx += 1;
            let delim = chars.next();
            if delim.is_none() {
                break;
            }

            let val = chars.next().unwrap();
            if delim == Some('[') {
                while self.0.len() <= idx {
                    self.0.push(Vec::new());
                }

                self.0[idx].push(val)
            }

            chars.next();
            chars.next();
        }
    }

    fn move_one_at_a_time(&mut self, m: Move) {
        for _ in 0..m.n {
            let v = self.0[m.from].pop().unwrap();
            self.0[m.to].push(v);
        }
    }

    fn move_n_at_a_time(&mut self, m: Move) {
        self.move_one_at_a_time(Move {
            from: m.from,
            to: 0,
            n: m.n,
        });

        self.move_one_at_a_time(Move {
            from: 0,
            to: m.to,
            n: m.n,
        })
    }

    fn top_crates(&self) -> String {
        self.0
            .iter()
            .skip(1)
            .map(|v| v.last().copied())
            .map(Option::unwrap)
            .collect()
    }
}

struct Move {
    from: usize,
    to: usize,
    n: usize,
}

impl From<String> for Move {
    fn from(value: String) -> Self {
        let mut vals = value
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(must_parse::<usize>);

        let n = vals.next().unwrap();
        let from = vals.next().unwrap();
        let to = vals.next().unwrap();
        Self { n, from, to }
    }
}

fn solve_1(input: &str) -> String {
    let mut lines = file_lines(input);
    let mut stacks = Stack::new(&mut lines);

    for m in lines.map(Move::from) {
        stacks.move_one_at_a_time(m);
    }

    stacks.top_crates()
}

fn solve_2(input: &str) -> String {
    let mut lines = file_lines(input);
    let mut stacks = Stack::new(&mut lines);

    for m in lines.map(Move::from) {
        stacks.move_n_at_a_time(m);
    }

    stacks.top_crates()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec05/example_1.txt";
        assert_eq!("CMZ", solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec05/input_1.txt";
        assert_eq!("JRVNHHCSJ", solve_1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec05/example_1.txt";
        assert_eq!("MCD", solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec05/input_1.txt";
        assert_eq!("GNFBSBJLH", solve_2(input));
    }
}
