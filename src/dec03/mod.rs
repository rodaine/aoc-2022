use crate::utils::*;
use std::collections::HashSet;
use std::hash::Hash;

fn char_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn priority(ch: char) -> u32 {
    if ch.is_ascii_lowercase() {
        ch as u32 - 96
    } else {
        ch as u32 - 65 + 27
    }
}

fn two_way_intersection<T>(a: HashSet<T>, b: HashSet<T>) -> impl Iterator<Item = T>
where
    T: Eq + Hash,
{
    a.into_iter().filter(move |el| b.contains(el))
}

fn three_way_intersection<T>(a: HashSet<T>, b: HashSet<T>, c: HashSet<T>) -> impl Iterator<Item = T>
where
    T: Eq + Hash,
{
    two_way_intersection(a, b).filter(move |el| c.contains(el))
}

fn solve_1(input: &str) -> u32 {
    file_lines(input)
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            (char_set(a), char_set(b))
        })
        .flat_map(|(a, b)| two_way_intersection(a, b))
        .map(priority)
        .sum()
}

fn solve_2(input: &str) -> u32 {
    file_lines(input)
        .map(|l| char_set(&l))
        .array_chunks()
        .flat_map(|[a, b, c]| three_way_intersection(a, b, c))
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec03/example_1.txt";
        assert_eq!(157, solve_1(input))
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec03/input_1.txt";
        assert_eq!(7581, solve_1(input))
    }

    #[test]
    fn example_2() {
        let input = "src/dec03/example_1.txt";
        assert_eq!(70, solve_2(input))
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec03/input_1.txt";
        assert_eq!(2525, solve_2(input))
    }
}
