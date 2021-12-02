use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn parse_input<T: FromStr>(input: String) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| line.parse::<T>().unwrap())
        .collect()
}

pub fn from_file<T: FromStr>(file: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    parse_input(fs::read_to_string(file).unwrap())
}
