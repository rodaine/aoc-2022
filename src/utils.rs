use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn load_file<P>(p: P) -> BufReader<File>
where
    P: AsRef<Path>,
{
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(p);
    let f = File::open(d).expect("failed to open file");
    BufReader::new(f)
}

pub fn file_lines<P>(p: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    load_file(p).lines().map(Result::unwrap)
}

pub fn must_parse<T>(v: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    v.parse::<T>().unwrap()
}

pub fn parse_items<T>(i: impl Iterator<Item = String>) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    i.map(|s| must_parse(&s))
}

pub fn file_groups<P>(p: P) -> impl Iterator<Item = impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let inner = file_lines(p);
    GroupIterator { inner }
}

pub struct GroupIterator<I>
where
    I: Iterator<Item = String>,
{
    inner: I,
}

impl<I: Iterator<Item = String>> Iterator for GroupIterator<I> {
    type Item = impl Iterator<Item = String>;

    fn next(&mut self) -> Option<Self::Item> {
        let group: Vec<String> = (&mut self.inner).take_while(|l| !l.is_empty()).collect();

        if group.is_empty() {
            return None;
        }

        Some(group.into_iter())
    }
}
