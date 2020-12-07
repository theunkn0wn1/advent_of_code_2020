#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

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
    let lines = line_reader.collect::<Result<Vec<_>, _>>()?;


    let element_iter = lines.split(|str| str == "");
    let foo = element_iter.collect_vec();
    for line in foo{
        let result = convert_entry_to_dict(line)?;
    }

    // for element in element_iter{
    //     let foo = element.iter().collect_vec();
    //     println!("{:?}", foo);
    // }
    Ok(())
}

fn convert_entry_to_dict(data: &[String]) -> anyhow::Result<HashMap<&str, &str>> {
    let mut formed_data = HashMap::new();
    println!("--------------");
    for line in data.into_iter() {
        // println!("raw line := {:?}", line);
        for element in line.split_ascii_whitespace(){
            // println!("raw element := {:?}", element);
            for (key, value) in element.split(":").collect_tuple() {
                // println!("key := {:?}\t value = {:?}", key, value);
                formed_data.insert(key, value);
            }
        }
    }
    println!("formed data := {:?}", formed_data);
    Ok(formed_data)
}