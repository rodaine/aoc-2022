use crate::utils::*;
use std::collections::VecDeque;
use std::io::Read;

fn find_distinct_run(input: &str, n: usize) -> usize {
    let chars = load_file(input).bytes().map(Result::unwrap).enumerate();

    let mut vd = VecDeque::with_capacity(n);

    for (idx, ch) in chars {
        if let Some(idx) = vd.iter().position(|&c| c == ch) {
            for _ in 0..=idx {
                vd.pop_front();
            }
        }

        vd.push_back(ch);
        if vd.len() == n {
            return idx + 1;
        }
    }

    unreachable!()
}

fn solve_1(input: &str) -> usize {
    find_distinct_run(input, 4)
}

fn solve_2(input: &str) -> usize {
    find_distinct_run(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(7, solve_1("src/dec06/example_1.txt"));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec06/input_1.txt";
        assert_eq!(1723, solve_1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec06/example_1.txt";
        assert_eq!(19, solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec06/input_1.txt";
        assert_eq!(3708, solve_2(input));
    }
}
