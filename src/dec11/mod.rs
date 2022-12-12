use crate::utils::*;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
enum Op {
    AddOld,
    Add(usize),
    MulOld,
    Mul(usize),
}

impl Op {
    fn apply(self, old: usize) -> usize {
        use Op::*;
        match self {
            AddOld => (old) + (old),
            Add(n) => (old) + (n),
            MulOld => (old) * (old),
            Mul(n) => (old) * (n),
        }
    }

    fn apply_mod(self, old: usize, modulo: usize) -> usize {
        use Op::*;
        (match self {
            AddOld => (old % modulo) + (old % modulo),
            Add(n) => (old % modulo) + (n % modulo),
            MulOld => (old % modulo) * (old % modulo),
            Mul(n) => (old % modulo) * (n % modulo),
        }) % modulo
    }
}

impl From<String> for Op {
    fn from(value: String) -> Self {
        let [o, v] = value.split_whitespace().skip(4).next_chunk().unwrap();

        use Op::*;
        match (o, v) {
            ("+", "old") => AddOld,
            ("+", v) => Add(must_parse(v)),
            ("*", "old") => MulOld,
            ("*", v) => Mul(must_parse(v)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test: (usize, usize, usize),
    processed_ct: usize,
    modulo: usize,
}

impl Monkey {
    fn turn(&mut self, output: &mut VecDeque<(usize, usize)>, anxiety_div: usize) {
        for old_item in self.items.drain(..) {
            self.processed_ct += 1;

            let new_item = if anxiety_div != 1 {
                self.op.apply(old_item) / anxiety_div
            } else {
                self.op.apply_mod(old_item, self.modulo)
            };

            let (div, tm, fm) = self.test;
            if new_item % div == 0 {
                output.push_back((tm, new_item));
            } else {
                output.push_back((fm, new_item));
            }
        }
    }
}

impl FromIterator<String> for Monkey {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut i = iter.into_iter().skip(1);

        let items: VecDeque<usize> = i
            .next()
            .unwrap()
            .split_whitespace()
            .skip(2)
            .map(|s: &str| s.strip_suffix(',').unwrap_or(s))
            .map(must_parse)
            .collect();

        let op = Op::from(i.next().unwrap());

        let div: usize = must_parse(i.next().unwrap().split_whitespace().last().unwrap());
        let tm: usize = must_parse(i.next().unwrap().split_whitespace().last().unwrap());
        let fm: usize = must_parse(i.next().unwrap().split_whitespace().last().unwrap());
        let test = (div, tm, fm);

        Self {
            items,
            op,
            test,
            processed_ct: 0,
            modulo: div,
        }
    }
}

#[derive(Debug)]
struct Barrel {
    monkeys: Vec<Monkey>,
    output: VecDeque<(usize, usize)>,
}

impl Barrel {
    fn run(&mut self, runs: usize, anxiety_div: usize) {
        (0..runs).for_each(|_| {
            self.round(anxiety_div);
        })
    }

    fn round(&mut self, anxiety_div: usize) {
        for idx in 0..self.monkeys.len() {
            self.monkeys[idx].turn(&mut self.output, anxiety_div);
            for (midx, item) in self.output.drain(..) {
                self.monkeys[midx].items.push_back(item);
            }
        }
    }
}

impl<I: Iterator<Item = String>> FromIterator<I> for Barrel {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut monkeys: Vec<Monkey> = iter.into_iter().map(Monkey::from_iter).collect();

        let modulo: usize = monkeys.iter().map(|m| m.test.0).product();
        for monkey in &mut monkeys {
            monkey.modulo = modulo;
        }

        Self {
            monkeys,
            output: VecDeque::new(),
        }
    }
}

fn solve(input: &str, runs: usize, anxiety_div: usize) -> usize {
    let mut barrel = Barrel::from_iter(file_groups(input));
    barrel.run(runs, anxiety_div);

    let mut out: Vec<usize> = barrel.monkeys.into_iter().map(|m| m.processed_ct).collect();
    out.sort();
    out.reverse();
    out[0] * out[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec11/example_1.txt";
        assert_eq!(10605, solve(input, 20, 3));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec11/input_1.txt";
        assert_eq!(99852, solve(input, 20, 3));
    }

    #[test]
    fn example_2() {
        let input = "src/dec11/example_1.txt";
        assert_eq!(2713310158, solve(input, 10_000, 1));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec11/input_1.txt";
        assert_eq!(25935263541, solve(input, 10_000, 1));
    }
}
