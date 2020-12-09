extern crate im_rc;

use im_rc::Vector;
use std::env;
use std::fs;
use std::io::Error;

pub fn read_integers_from_param() -> Vector<i64> {
    read_integers(&input_file_from_param()).expect("Failed to read integers fro input file")
}

pub fn read_integers(path: &str) -> Result<Vector<i64>, Error> {
    read_strings(path).map(|lines| {
        lines
            .iter()
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    })
}

pub fn parse_integers(input: &str) -> Vector<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

pub fn read_strings(path: &str) -> Result<Vector<String>, Error> {
    read_string(path).map(|data| data.lines().map(|line| line.to_owned()).collect())
}

pub fn read_string(path: &str) -> Result<String, Error> {
    println!("Reading data from {}", path);
    fs::read_to_string(path)
}

pub fn read_strings_from_param() -> Vector<String> {
    read_strings(&input_file_from_param()).expect("Failed to read input file")
}

pub fn read_string_from_params() -> String {
    read_string(&input_file_from_param()).expect("Failed to read input file")
}

pub fn input_file_from_param() -> String {
    env::args()
        .nth(1)
        .expect("Pass the input file as first parameter")
}
