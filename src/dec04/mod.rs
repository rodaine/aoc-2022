use crate::utils::*;
use std::ops::RangeInclusive;

struct Pair(RangeInclusive<u32>, RangeInclusive<u32>);

impl Pair {
    fn includes(outer: &RangeInclusive<u32>, inner: &RangeInclusive<u32>) -> bool {
        outer.contains(inner.start()) && outer.contains(inner.end())
    }

    fn overlaps(first: &RangeInclusive<u32>, second: &RangeInclusive<u32>) -> bool {
        first.contains(second.start()) || first.contains(second.end())
    }

    fn any_overlap(&self) -> bool {
        Self::overlaps(&self.0, &self.1) || Self::overlaps(&self.1, &self.0)
    }

    fn full_overlap(&self) -> bool {
        Self::includes(&self.0, &self.1) || Self::includes(&self.1, &self.0)
    }
}

impl From<String> for Pair {
    fn from(value: String) -> Self {
        let mut iter = value
            .splitn(4, &[',', '-'])
            .array_chunks()
            .map(|[start, end]| must_parse::<u32>(start)..=must_parse::<u32>(end));
        Pair(iter.next().unwrap(), iter.next().unwrap())
    }
}

fn solve1(input: &str) -> usize {
    file_lines(input)
        .map(Pair::from)
        .filter(Pair::full_overlap)
        .count()
}

fn solve2(input: &str) -> usize {
    file_lines(input)
        .map(Pair::from)
        .filter(Pair::any_overlap)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec04/example_1.txt";
        assert_eq!(2, solve1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec04/input_1.txt";
        assert_eq!(515, solve1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec04/example_1.txt";
        assert_eq!(4, solve2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec04/input_1.txt";
        assert_eq!(883, solve2(input));
    }
}
