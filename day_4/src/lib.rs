#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::{read_entries, convert_entry_to_dict, solve_p1};
    use std::collections::HashMap;

    #[test]
    fn test_read_entries() {
        read_entries("sample.txt").expect("failed.");
    }

    #[test]
    fn test_convert_to_dict() {
        let sample: Vec<String> = vec![
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm")
        ];

        let mut expected = HashMap::new();
        expected.insert("ecl", "gry");
        expected.insert("pid", "860033327");
        expected.insert("eyr", "2020");
        expected.insert("hcl", "#fffffd");

        expected.insert("byr", "1937");
        expected.insert("iyr", "2017");
        expected.insert("cid", "147");
        expected.insert("hgt", "183cm");

        let result = convert_entry_to_dict(sample.as_slice()).expect("failed!");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_p1() {
        let validated = solve_p1("sample.txt").expect("failed!");
        assert_eq!(validated, 2);
    }
}

fn read_entries(path: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let handle = File::open(path)?;
    let line_reader = io::BufReader::new(handle).lines();
    let lines = line_reader.collect::<Result<Vec<_>, _>>()?;


    let element_iter = lines.split(|str| str == "").map(|item| item.to_owned());
    let foo = element_iter.collect_vec();
    Ok(foo)
}

fn convert_entry_to_dict(data: &[String]) -> anyhow::Result<HashMap<&str, &str>> {
    let mut formed_data = HashMap::new();
    // println!("--------------");
    for line in data.into_iter() {
        // println!("raw line := {:?}", line);
        for element in line.split_ascii_whitespace() {
            // println!("raw element := {:?}", element);
            for (key, value) in element.split(":").collect_tuple() {
                // println!("key := {:?}\t value = {:?}", key, value);
                formed_data.insert(key, value);
            }
        }
    }
    // println!("formed data := {:?}", formed_data);
    Ok(formed_data)
}

fn validate_p1(data: HashMap<&str, &str>) -> anyhow::Result<bool> {
    if !data.contains_key("byr") {
        return Ok(false);
    }
    if !data.contains_key("iyr") {
        return Ok(false);
    }
    if !data.contains_key("eyr") {
        return Ok(false);
    }
    if !data.contains_key("hgt") {
        return Ok(false);
    }
    if !data.contains_key("hcl") {
        return Ok(false);
    }
    if !data.contains_key("ecl") {
        return Ok(false);
    }
    if !data.contains_key("pid") {
        return Ok(false);
    }
    // if !data.contains_key("cid") {
    //     return Ok(false);
    // }
    Ok(true)
}

pub fn solve_p1(path: &str) -> anyhow::Result<usize> {
    let entries = read_entries(path)?;
    let mut valid = 0;
    for entry in entries {
        let entry_dict = convert_entry_to_dict(&entry)?;
        if validate_p1(entry_dict)? {
            valid += 1;
        }
    }

    Ok(valid)
}