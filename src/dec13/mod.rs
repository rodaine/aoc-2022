use crate::utils::*;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketDatum {
    Int(usize),
    List(Vec<PacketDatum>),
}

impl PacketDatum {
    fn parse_list(chars: &mut Peekable<Chars>) -> Self {
        chars.next(); // [

        let mut list = Vec::new();
        while let Some(&ch) = chars.peek() {
            match ch {
                ',' => {
                    chars.next();
                }
                ']' => {
                    chars.next(); // ]
                    return PacketDatum::List(list);
                }
                '[' => list.push(Self::parse_list(chars)),
                _ => list.push(Self::parse_int(chars)),
            }
        }

        unreachable!()
    }

    fn parse_int(chars: &mut Peekable<Chars>) -> Self {
        let mut digits = Vec::new();
        while chars.peek().unwrap().is_ascii_digit() {
            digits.push(chars.next().unwrap());
        }

        let s: String = digits.iter().collect();
        PacketDatum::Int(must_parse(&s))
    }
}

impl From<String> for PacketDatum {
    fn from(value: String) -> Self {
        let mut chars = value.chars().peekable();
        PacketDatum::parse_list(&mut chars)
    }
}

impl PartialOrd for PacketDatum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use PacketDatum::*;

        match (self, other) {
            (Int(l), Int(r)) => l.partial_cmp(r),
            (List(l), List(r)) => l.partial_cmp(r),
            (Int(l), List(r)) => (vec![Int(*l)]).partial_cmp(r),
            (List(l), Int(r)) => l.partial_cmp(&vec![Int(*r)]),
        }
    }
}

impl Ord for PacketDatum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Pair(PacketDatum, PacketDatum);

impl FromIterator<String> for Pair {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        Self(
            PacketDatum::from(i.next().unwrap()),
            PacketDatum::from(i.next().unwrap()),
        )
    }
}

fn solve_1(input: &str) -> usize {
    file_groups(input)
        .map(Pair::from_iter)
        .enumerate()
        .filter(|(_, p)| p.0 <= p.1)
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_2(input: &str) -> usize {
    use PacketDatum::*;
    let start = List(vec![List(vec![Int(2)])]);
    let end = List(vec![List(vec![Int(6)])]);

    let mut packets: Vec<PacketDatum> = file_lines(input)
        .filter(|s| !s.is_empty())
        .map(PacketDatum::from)
        .collect();

    packets.push(start.clone());
    packets.push(end.clone());
    packets.sort();

    let s = packets.binary_search(&start).unwrap() + 1;
    let e = packets.binary_search(&end).unwrap() + 1;
    s * e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec13/example_1.txt";
        assert_eq!(13, solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec13/input_1.txt";
        assert_eq!(5808, solve_1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec13/example_1.txt";
        assert_eq!(140, solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec13/input_1.txt";
        assert_eq!(22713, solve_2(input));
    }
}
