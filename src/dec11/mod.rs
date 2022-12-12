use std::collections::VecDeque;
use crate::utils::*;

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
        (match self {
            AddOld => (old ) + (old ),
            Add(n) => (old ) + (n ),
            MulOld => (old ) * (old ),
            Mul(n) => (old ) * (n),
        })
    }
}

impl From<String> for Op {
    fn from(value: String) -> Self {
        let [o, v] =  value.trim()
            .split_whitespace()
            .skip(4)
            .next_chunk()
            .unwrap();

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
}

impl Monkey {
    fn turn(&mut self, output: &mut VecDeque<(usize, usize)>, anxiety_div: usize) {
        for mut item in self.items.drain(..) {
            self.processed_ct += 1;

            item = self.op.apply(item);
            item /= anxiety_div;

            let (div, tm, fm) = self.test;
            if item % div == 0 {
                output.push_back((tm, item));
            } else {
                output.push_back((fm, item));
            }
        }
    }
}

impl FromIterator<String> for Monkey {
    fn from_iter<T: IntoIterator<Item=String>>(iter: T) -> Self {
        let mut i = iter.into_iter().skip(1);

        let items: VecDeque<usize> = i.next().unwrap().trim()
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
            processed_ct:0,
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
        (0..runs).for_each(|_|self.round(anxiety_div))
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

impl<I:Iterator<Item=String>> FromIterator<I> for Barrel {
    fn from_iter<T: IntoIterator<Item=I>>(iter: T) -> Self {
        let mut monkeys : Vec<Monkey> = iter.into_iter().map(Monkey::from_iter).collect();
        Self {monkeys, output: VecDeque::new()}
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
        assert_eq!(0, solve(input, 10_000, 1));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec11/input_1.txt";
        assert_eq!(0, solve(input, 10_000, 1));
    }
}
