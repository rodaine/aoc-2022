use crate::utils::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    size: usize,
}

impl Dir {
    fn root() -> Self {
        Self {
            parent: None,
            size: 0,
        }
    }

    fn new(parent: Rc<RefCell<Dir>>) -> Self {
        Self {
            parent: Some(parent),
            size: 0,
        }
    }

    fn add_size(&mut self, size: usize) {
        self.size += size;
        if self.parent.is_none() {
            return;
        }
        self.parent.as_ref().unwrap().borrow_mut().add_size(size);
    }
}

#[derive(Debug)]
struct File {
    size: usize,
    parent: Rc<RefCell<Dir>>,
}

#[derive(Debug)]
struct FS {
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, File>,
}

impl FS {
    fn new() -> Self {
        let mut fs = Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
        };
        let root = Dir::root();
        fs.dirs.insert("/".to_owned(), Rc::new(RefCell::new(root)));
        fs
    }

    fn add_dir(&mut self, parent: &str, name: &str) {
        let d = Dir::new(Rc::clone(self.dirs.get(parent).unwrap()));
        self.dirs
            .insert(Self::full_name(parent, name), Rc::new(RefCell::new(d)));
    }

    fn add_file(&mut self, parent: &str, name: &str, size: usize) {
        let p = Rc::clone(self.dirs.get(parent).unwrap());
        p.borrow_mut().add_size(size);
        self.files
            .insert(Self::full_name(parent, name), File { parent: p, size });
    }

    fn full_name(parent: &str, name: &str) -> String {
        if parent == "/" {
            format!("/{name}")
        } else {
            format!("{parent}/{name}")
        }
    }

    fn sum_of_dirs_at_most(&self, size: usize) -> usize {
        self.dirs
            .iter()
            .filter(|&(name, _)| name != "/")
            .map(|(_, d)| d.borrow().size)
            .filter(|&s| s <= size)
            .sum()
    }

    fn min_to_delete(&self, target: usize) -> usize {
        let at_least = self.dirs.get("/").unwrap().borrow().size - target;
        self.dirs.values()
            .map(|d| d.borrow().size)
            .filter(|&s| s >= at_least)
            .min()
            .unwrap()
    }
}

impl FromIterator<String> for FS {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut fs = FS::new();
        let mut current = PathBuf::new();

        let mut i = iter.into_iter().peekable();
        while let Some(line) = i.next() {
            let mut parts = line.split_whitespace().skip(1);

            match parts.next().unwrap() {
                "cd" => match parts.next().unwrap() {
                    ".." => {
                        current.pop();
                    }
                    d => {
                        current.push(d);
                    }
                },
                "ls" => {
                    while !i.peek().map_or(true, |l| l.starts_with('$')) {
                        let sl: String = i.next().unwrap();
                        let mut parts = sl.split_whitespace();
                        let parent = current.to_str().unwrap();

                        match parts.next().unwrap() {
                            "dir" => fs.add_dir(parent, parts.next().unwrap()),
                            s => fs.add_file(parent, parts.next().unwrap(), must_parse(s)),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        fs
    }
}

fn solve_1(input: &str) -> usize {
    let fs = FS::from_iter(file_lines(input));
    fs.sum_of_dirs_at_most(100_000)
}

fn solve_2(input: &str) -> usize {
    let fs = FS::from_iter(file_lines(input));
    fs.min_to_delete(40_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "src/dec07/example_1.txt";
        assert_eq!(95437, solve_1(input));
    }

    #[test]
    fn puzzle_1() {
        let input = "src/dec07/input_1.txt";
        assert_eq!(1723892, solve_1(input));
    }

    #[test]
    fn example_2() {
        let input = "src/dec07/example_1.txt";
        assert_eq!(24933642, solve_2(input));
    }

    #[test]
    fn puzzle_2() {
        let input = "src/dec07/input_1.txt";
        assert_eq!(8474158, solve_2(input));
    }
}
