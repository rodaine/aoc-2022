use crate::utils::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(self) -> Self {
        let val: isize = self.into();
        Self::from(val-1)
    }

    fn loses_to(self) -> Self {
        let val: isize = self.into();
        Self::from(val+1)
    }
}

impl From<Shape> for isize {
    fn from(value: Shape) -> Self {
        use Shape::*;
        match value {
            Rock => 0,
            Paper => 1,
            Scissors => 2,
        }
    }
}

impl From<isize> for Shape {
    fn from(value: isize) -> Self {
        use Shape::*;
        match (value+3)%3 {
            0 => Rock,
            1 => Paper,
            2 => Scissors,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        use Shape::*;
        match value {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<Outcome> for isize {
    fn from(value: Outcome) -> isize {
        use Outcome::*;
        match value {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        use Outcome::*;
        match value {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!(),
        }
    }
}

impl From<Pair> for Outcome {
    fn from(value: Pair) -> Outcome {
        use Outcome::*;
        let Pair(opp, me) = value;

        if opp == me {
            return Draw
        }

        if me.beats() == opp {
            return Win
        }

        Lose
    }
}

#[derive(Copy, Clone)]
struct Pair(Shape, Shape);

impl From<String> for Pair {
    fn from(value: String) -> Self {
        let (o, m) = value.split_once(' ').unwrap();
        Pair(o.into(), m.into())
    }
}

impl From<Pair> for isize {
    fn from(value: Pair) -> Self {
        let outcome: Outcome = value.into();
        score(value.1, outcome)
    }
}

struct Target(Shape, Outcome);

impl From<String> for Target {
    fn from(value: String) -> Self {
        let (opp, out) = value.split_once(' ').unwrap();
        Target(opp.into(), out.into())
    }
}

impl From<Target> for isize {
    fn from(value: Target) -> Self {
        use Outcome::*;

        let Target(opp, out) = value;
        let me = match out {
            Lose => opp.beats(),
            Draw => opp,
            Win => opp.loses_to(),
        };

        score(me, out)
    }
}

fn score(me: Shape, out: Outcome) -> isize {
    let m: isize = me.into();
    let o: isize = out.into();
    1 + m + o
}

fn solve1(input: &str) -> isize {
    file_lines(input)
        .map(Pair::from)
        .map(isize::from)
        .sum()
}

fn solve2(input: &str) -> isize {
    file_lines(input)
        .map(Target::from)
        .map(isize::from)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec02/example_1.txt";
        assert_eq!(15, solve1(input))
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec02/input_1.txt";
        assert_eq!(11841, solve1(input))
    }

    #[test]
    fn example_2() {
        let input = "src/dec02/example_1.txt";
        assert_eq!(12, solve2(input))
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec02/input_1.txt";
        assert_eq!(13022, solve2(input))
    }
}