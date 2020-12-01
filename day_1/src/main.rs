use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;
use rayon::prelude::*;

fn do_read() -> Result<Vec<i32>> {
    let mut numbers: Vec<i32> = Vec::new();
    println!("reading file...");
    let lines = read_lines("./day_1/input.txt")?;
    for line_result in lines {
        let line = line_result?;
        let converted: i32 = line.parse()?;
        numbers.push(converted);
    };
    println!("done reading lines. i have {} lines.", numbers.len());
    Ok(numbers)
}

fn main() -> Result<()> {
    let numbers = do_read()?;

    println!("=== part one ===");
    numbers2(numbers.clone());
    println!("--- part two ---");
    numbers3(numbers);
    println!("Done.");
    Ok(())
}

fn numbers2(numbers: Vec<i32>) {
    numbers.par_iter().for_each(
        |x| {
            numbers.par_iter().filter(|y| { x + *y == 2020 }).for_each(|y| {
                println!("{} + {} = {}", x, y, x + y);
                println!("{} * {} = {}", x, y, x * y);
            });
        }
    );
}

fn numbers3(numbers: Vec<i32>) {
    numbers.par_iter().for_each(
        |x| {
            numbers.par_iter().for_each(|y| {
                numbers.par_iter().filter(|z| { x + y + *z == 2020 }).for_each(|z| {
                    println!("{} + {} + {} = {}", x, y, z, x + z);
                    println!("{} * {} * {}= {}", x, y, z, x * y * z);
                });
            })
        }
    );
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

