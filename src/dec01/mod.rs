use crate::utils::*;
use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;

fn total_calories(input: &str) -> impl Iterator<Item = usize> + '_ {
    file_groups(input)
        .map(parse_items::<usize>)
        .map(Iterator::sum)
}

fn solve_1(input: &str) -> usize {
    total_calories(input).max().unwrap()
}

fn solve_2(input: &str, n: usize) -> usize {
    let mut heap = BinaryHeap::with_capacity(n);
    let cals = total_calories(input).map(Reverse);
    for ttl in cals {
        if heap.len() >= n {
            if let Some(Reverse(prev)) = heap.peek() && *prev >= ttl.0 {
                continue;
            }
            heap.pop();
        }
        heap.push(ttl);
    }

    heap.into_iter().map(|r| r.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec01/example_1.txt";
        assert_eq!(24000, solve_1(input))
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec01/input_1.txt";
        assert_eq!(71780, solve_1(input))
    }

    #[test]
    fn example_2() {
        let input = "src/dec01/example_1.txt";
        assert_eq!(45000, solve_2(input, 3))
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec01/input_1.txt";
        assert_eq!(212489, solve_2(input, 3))
    }
}
