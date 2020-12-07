#[allow(dead_code)]
#[allow(unused_variables)]
use std::fs::{File, read};
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::read_entries;

    #[test]
    fn test_read_entries() {
        read_entries().expect("failed.");
    }
}

fn read_entries() -> anyhow::Result<()> {
    let handle = File::open("sample.txt")?;
    let line_reader = io::BufReader::new(handle).lines();
    let lines = line_reader.collect::<Result<Vec<_>, _>>().unwrap();


    let element_iter = lines.split(|str| str == "");
    let elements = element_iter.collect_vec();
    println!("{:?}", elements);
    Ok(())
}