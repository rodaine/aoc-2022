use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use crate::utils::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Idx(isize, isize);

impl Idx {
    fn distance(&self, other: &Self) -> isize {
        (other.0 - self.0).abs() + (other.1 - self.1).abs()
    }

    fn intercept(&self, y: isize) -> Idx {
        Self(self.0, y)
    }

    fn min(&self, other: Self) -> Self {
        Self(
            self.0.min(other.0),
            self.1.min(other.1),
        )
    }

    fn max(&self, other: Self) -> Self {
        Self(
            self.0.max(other.0),
            self.1.max(other.1),
        )
    }

    fn row_coverage(&self, beacon: &Self, y: isize) -> Option<RangeInclusive<isize>> {
        let dist = self.distance(beacon);
        let intercept = self.intercept(y);
        let used = self.distance(&intercept);
        let remaining = dist - used;

        if remaining < 0 {
            return None;
        }

        Some((intercept.0 - remaining)..=(intercept.0 + remaining))
    }

    const MIN: Self = Self(isize::MIN, isize::MIN);
    const MAX: Self = Self(isize::MAX, isize::MAX);
}

struct Space {
    min: Idx,
    max: Idx,

    sensors: HashMap<isize, HashMap<isize, Idx>>, // y -> x -> nearest beacon
    beacons: HashMap<isize, HashMap<isize, Vec<Idx>>>, // y -> x -> nearest sensors
}

#[derive(Eq, PartialEq, Clone)]
struct Coverage(RangeInclusive<isize>);

impl Coverage {
    fn contains(&self, value: &isize) -> bool {
        self.0.contains(value)
    }
}

impl PartialOrd for Coverage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.0.start(), self.0.end()).partial_cmp(&(other.0.start(), other.0.end()))
    }
}

impl Ord for Coverage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Space {
    fn read_line_density<T: IntoIterator<Item=String>>(iter: T, y: isize) -> (Self, usize) {
        let mut space = Space::default();
        let mut beacons = HashSet::new();
        let mut coverage = Vec::new();

        for line in iter {
            let mut coords = line
                .split_ascii_whitespace()
                .skip(2)
                .filter(|w| w.starts_with(&['x', 'y']))
                .map(|w| w.split_once('=').unwrap())
                .map(|(_, n)| n.strip_suffix(&[',', ':']).unwrap_or(n))
                .map(|n| must_parse::<isize>(n));

            let sensor = Idx(coords.next().unwrap(), coords.next().unwrap());
            let beacon = Idx(coords.next().unwrap(), coords.next().unwrap());

            space.min = space.min.min(sensor.min(beacon));
            space.max = space.max.max(sensor.max(beacon));

            if beacon.1 == y {
                beacons.insert(beacon.0);
            }

            if let Some(cov) = sensor.row_coverage(&beacon, y) {
                space.min = space.min.min(Idx(*cov.start(), y));
                space.max = space.max.max(Idx(*cov.end(), y));
                coverage.push(Coverage(cov))
            }
        }

        coverage.sort();
        coverage.windows(2).flat_map(|&[a, b]| {
            if a.contains(b.0.start()) {
                vec![]
            } else {

            }
        });

        let density = (space.min.0..=space.max.0)
            .filter(|x| !beacons.contains(x))
            .filter(|x| coverage.iter().any(|r| r.contains(x)))
            .count();

        (space, density)
    }
}

impl Default for Space {
    fn default() -> Self {
        Self {
            min: Idx::MAX,
            max: Idx::MIN,
            sensors: HashMap::new(),
            beacons: HashMap::new(),
        }
    }
}

fn solve_1(input: &str, y: isize) -> usize {
    let (space, density) = Space::read_line_density(file_lines(input), y);
    density
}

fn solve_2(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec15/example_1.txt";
        assert_eq!(26, solve_1(input, 10));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec15/input_1.txt";
        assert_eq!(4883971, solve_1(input,2_000_000));
    }

    #[test]
    fn example_2() {
        let input = "src/dec15/example_1.txt";
        assert_eq!(0, solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec15/input_1.txt";
        assert_eq!(0, solve_2(input));
    }
}
