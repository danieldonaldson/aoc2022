#![feature(array_windows)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let lines_as_vector: Vec<_> = lines
            .map(|l| l.expect("error").parse::<i32>().unwrap())
            .collect();
        let window_of_3: Vec<_> = lines_as_vector
            .array_windows()
            .map(|[a, b, c]| a + b + c)
            .collect();
        let sum_of_window = window_of_3.array_windows().filter(|[a, b]| b > a).count();
        dbg!(window_of_3);
        dbg!(sum_of_window);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
