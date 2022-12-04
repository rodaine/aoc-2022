use crate::utils::*;

#[derive(Copy, Clone)]
struct Shape(i8);

impl Shape {
    fn against(self, opp: Self) -> Outcome {
        Outcome((self.0 - opp.0).rem_euclid(3))
    }

    fn my_play(self, out: Outcome) -> Self {
        Self((self.0 + out.0).rem_euclid(3))
    }

    fn score(self) -> isize {
        (1 + self.0) as isize
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self(0), // rock
            "B" | "Y" => Self(1), // paper
            "C" | "Z" => Self(2), // scissors
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
struct Outcome(i8);

impl Outcome {
    fn score(self) -> isize {
        (((1 + self.0) % 3) * 3) as isize
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self(2), // lose
            "Y" => Self(0), // draw
            "Z" => Self(1), // win
            _ => unreachable!(),
        }
    }
}

fn solve_1(input: &str) -> isize {
    file_lines(input)
        .map(|s| {
            let (opp, me) = s.split_once(' ').unwrap();
            (Shape::from(opp), Shape::from(me))
        })
        .map(|(opp, me)| (me, me.against(opp)))
        .map(|(me, out)| me.score() + out.score())
        .sum()
}

fn solve_2(input: &str) -> isize {
    file_lines(input)
        .map(|s| {
            let (opp, out) = s.split_once(' ').unwrap();
            (Shape::from(opp), Outcome::from(out))
        })
        .map(|(opp, out)| (opp.my_play(out), out))
        .map(|(me, out)| me.score() + out.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec02/example_1.txt";
        assert_eq!(15, solve_1(input))
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec02/input_1.txt";
        assert_eq!(11841, solve_1(input))
    }

    #[test]
    fn example_2() {
        let input = "src/dec02/example_1.txt";
        assert_eq!(12, solve_2(input))
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec02/input_1.txt";
        assert_eq!(13022, solve_2(input))
    }
}
