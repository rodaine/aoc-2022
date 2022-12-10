use crate::utils::*;
use std::collections::{HashMap, VecDeque};

struct Cpu {
    cycle: isize,
    x: isize,
    capture_points: VecDeque<isize>,
    captures: HashMap<isize, isize>,
    crt: String,
}

impl Cpu {
    fn new(capture_points: VecDeque<isize>) -> Self {
        let captures = HashMap::with_capacity(capture_points.len());
        Self {
            cycle: 1,
            x: 1,
            crt: String::new(),
            capture_points,
            captures,
        }
    }

    fn read_inst(&mut self, inst: &str) {
        match inst.split_once(' ') {
            None if inst == "noop" => self.step(),
            Some((op, n)) if op == "addx" => {
                self.step();
                self.step();
                let d: isize = must_parse(n);
                self.x += d;
            }
            _ => unreachable!(),
        }
    }

    fn step(&mut self) {
        self.draw();

        if let Some(&pt) = self.capture_points.front() && pt == self.cycle {
            self.capture_points.pop_front();
            self.captures.insert(pt, self.x);
        }

        self.cycle += 1;
    }

    fn draw(&mut self) {
        let pos = (self.cycle - 1) % 40;
        if pos == 0 && self.cycle != 1 {
            self.crt.push('\n');
        }

        if (self.x - pos).abs() <= 1 {
            self.crt.push('#');
        } else {
            self.crt.push('.');
        }
    }
}

impl FromIterator<String> for Cpu {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let capture_points = VecDeque::from_iter::<[isize; 6]>([20, 60, 100, 140, 180, 220]);
        let mut cpu = Cpu::new(capture_points);
        iter.into_iter().for_each(|l| cpu.read_inst(&l));
        cpu
    }
}

fn solve_1(input: &str) -> isize {
    let cpu = Cpu::from_iter(file_lines(input));
    cpu.captures.iter().map(|(c, x)| *c * *x).sum()
}

fn solve_2(input: &str) -> String {
    let cpu = Cpu::from_iter(file_lines(input));
    cpu.crt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec10/example_1.txt";
        assert_eq!(13140, solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec10/input_1.txt";
        assert_eq!(15120, solve_1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec10/example_1.txt";
        let ex = "##..##..##..##..##..##..##..##..##..##..\n\
                        ###...###...###...###...###...###...###.\n\
                        ####....####....####....####....####....\n\
                        #####.....#####.....#####.....#####.....\n\
                        ######......######......######......####\n\
                        #######.......#######.......#######.....";

        assert_eq!(ex, &solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec10/input_1.txt";
        let ex = "###..#..#.###....##.###..###..#.....##..\n\
                        #..#.#.#..#..#....#.#..#.#..#.#....#..#.\n\
                        #..#.##...#..#....#.###..#..#.#....#..#.\n\
                        ###..#.#..###.....#.#..#.###..#....####.\n\
                        #.#..#.#..#....#..#.#..#.#....#....#..#.\n\
                        #..#.#..#.#.....##..###..#....####.#..#.";
        assert_eq!(ex, &solve_2(input));
    }
}
