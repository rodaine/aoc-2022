use crate::utils::*;

#[derive(Debug)]
struct Grid {
    heights: Vec<Vec<i8>>,
    visible: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn scan(&mut self) {
        self.scan_horiz();
        self.scan_vert();
    }

    fn scan_horiz(&mut self) {
        let w = (self.cols - 1) as isize;
        for r in 0..self.rows {
            let row = r as isize;
            self.scan_helper(row, 0, 0, 1);
            self.scan_helper(row, w, 0, -1);
        }
    }

    fn scan_vert(&mut self) {
        let h = (self.rows - 1) as isize;
        for c in 0..self.cols {
            let col = c as isize;
            self.scan_helper(0, col, 1, 0);
            self.scan_helper(h, col, -1, 0);
        }
    }

    fn scan_helper(&mut self, r: isize, c: isize, dr: isize, dc: isize) {
        let (mut row, mut col) = (r, c);
        let mut max = -1;
        while self.in_range(row, col) {
            let h = self.heights[row as usize][col as usize];
            if h > max {
                max = h;
                self.visible[row as usize][col as usize] = true;
            }
            if max == 9 {
                break;
            }

            row += dr;
            col += dc;
        }
    }

    fn in_range(&self, r: isize, c: isize) -> bool {
        r >= 0 && r < self.rows as isize && c >= 0 && c < self.cols as isize
    }

    fn total_visible(&self) -> usize {
        self.visible.iter().flatten().filter(|&&b| b).count()
    }

    fn scenic_score(&self, r: isize, c: isize) -> u32 {
        self.score_helper(r, c, -1, 0)
            * self.score_helper(r, c, 0, -1)
            * self.score_helper(r, c, 0, 1)
            * self.score_helper(r, c, 1, 0)
    }

    fn score_helper(&self, r: isize, c: isize, dr: isize, dc: isize) -> u32 {
        let (mut row, mut col) = (r + dr, c + dc);
        let target = self.heights[r as usize][c as usize];
        let mut ct = 0;

        while self.in_range(row, col) {
            let h = self.heights[row as usize][col as usize];
            ct += 1;
            if h >= target {
                break;
            }
            row += dr;
            col += dc;
        }

        ct
    }

    fn best_score(&self) -> u32 {
        (1..(self.rows - 1))
            .flat_map(|r| (1..(self.cols - 1)).map(move |c| (r, c)))
            .map(|(r, c)| self.scenic_score(r as isize, c as isize))
            .max()
            .unwrap()
    }
}

impl FromIterator<String> for Grid {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut heights = Vec::new();

        for line in iter.into_iter() {
            let row: Vec<i8> = line
                .chars()
                .map(|ch: char| ch.to_digit(10).unwrap() as i8)
                .collect();
            heights.push(row);
        }

        let rows = heights.len();
        let cols = heights[0].len();
        let visible = vec![vec![false; cols]; rows];
        Self {
            heights,
            visible,
            rows,
            cols,
        }
    }
}

fn solve_1(input: &str) -> usize {
    let mut grid = Grid::from_iter(file_lines(input));
    grid.scan();
    grid.total_visible()
}

fn solve_2(input: &str) -> u32 {
    let grid = Grid::from_iter(file_lines(input));
    grid.best_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec08/example_1.txt";
        assert_eq!(21, solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec08/input_1.txt";
        assert_eq!(1_870, solve_1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec08/example_1.txt";
        assert_eq!(8, solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec08/input_1.txt";
        assert_eq!(517_440, solve_2(input));
    }
}
