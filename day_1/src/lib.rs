use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;
use std::panic::resume_unwind;

pub fn do_read() -> Result<Vec<i32>> {
    let mut numbers: Vec<i32> = Vec::new();
    println!("reading file...");
    let lines = read_lines("input.txt")?;
    for line_result in lines {
        let line = line_result?;
        let converted: i32 = line.parse()?;
        numbers.push(converted);
    };
    println!("done reading lines. i have {} lines.", numbers.len());
    Ok(numbers)
}


pub fn numbers2(numbers: Vec<i32>) -> Option<i32> {
    let mut result = numbers.iter().map(
        |x| {
            numbers.iter().filter(|y| { x + *y == 2020 }).map(|y| {
                x * y
            }).next()
        }
    ).filter(|sum| match sum {
        None => { false }
        Some(_) => { true }
    });

    result.next().unwrap()
}

pub fn numbers3(numbers: Vec<i32>) -> Option<i32> {
    for x in numbers.iter(){
        for y in numbers.iter(){
            for z in numbers.iter(){
                if x+y+z == 2020 {
                    return Some(x*y*z);
                }
            }
        }
    }

    return None;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

