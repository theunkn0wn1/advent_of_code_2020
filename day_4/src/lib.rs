#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::{read_entries, convert_entry_to_dict, solve_p1, solve_p2};
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

    #[test]
    fn test_p2_invalid() {
        let validated = solve_p2("invalid.txt").expect("failed!");
        assert_eq!(validated, 0);
    }

    #[test]
    fn test_p2_valid() {
        let validated = solve_p2("valid.txt").expect("failed!");
        assert_eq!(validated, 4);
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

fn validate_p1(data: &HashMap<&str, &str>) -> anyhow::Result<bool> {
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

fn validate_p2(data: &HashMap<&str, &str>) -> anyhow::Result<bool> {
    // sanity check
    if !validate_p1(&data)? {
        return Ok(false);
    }
    let byr: u16 = data.get("byr").unwrap().parse()?;
    if byr < 1920 || byr > 2002 {
        return Ok(false);
    }
    let iyr: u16 = data.get("iyr").unwrap().parse()?;
    if iyr < 2010 || iyr > 2020 {
        return Ok(false);
    }
    let eyr: u16 = data.get("eyr").unwrap().parse()?;
    if eyr < 2020 || eyr > 2030 {
        return Ok(false);
    }
    let hgt = data.get("hgt").unwrap();

    if hgt.ends_with("cm") {
        let prefix: u16 = hgt.strip_suffix("cm").unwrap().parse()?;
        if prefix < 150 || prefix > 193 {
            return Ok(false);
        }
    } else if hgt.ends_with("in") {
        let prefix: u16 = hgt.strip_suffix("in").unwrap().parse()?;
        if prefix < 59 || prefix > 76 {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }


    // hcl
    let hcl: &&str = data.get("hcl").unwrap();
    // must be '#' prefixing six characters, therefore the length MUST be 7.
    if hcl.len() != 7 {
        return Ok(false);
    }
    // strip leading '#' prefix, if it doesn't exist its a fail.
    if !hcl.starts_with('#') {
        return Ok(false);
    }
    // this unwrap is safe as we validated the prefix existed.
    let hcl = hcl.strip_prefix('#').unwrap();

    // remainder should be a hex integer.
    let hcl: u32 = u32::from_str_radix(hcl, 16)?;

    // if we got this far, hcl is valid.
    let ecl: &&str = data.get("ecl").unwrap();
    if ecl.len() != 3 {
        return Ok(false);
    };
    const PERMISSIBLE: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if !PERMISSIBLE.contains(ecl) {
        return Ok(false);
    }

    let pid = data.get("pid").unwrap();

    if pid.len() != 9 {
        return Ok(false);
    };
    let _pid: u32 = pid.parse()?;


    // if it got this far, its valid.
    Ok(true)
}


pub fn solve_p1(path: &str) -> anyhow::Result<usize> {
    let entries = read_entries(path)?;
    let mut valid = 0;
    for entry in entries {
        let entry_dict = convert_entry_to_dict(&entry)?;
        if validate_p1(&entry_dict)? {
            valid += 1;
        }
    }

    Ok(valid)
}

pub fn solve_p2(path: &str) -> anyhow::Result<usize> {
    let entries = read_entries(path)?;
    let mut valid = 0;
    for entry in entries {
        let entry_dict = convert_entry_to_dict(&entry)?;
        if validate_p1(&entry_dict)? {
            if validate_p2(&entry_dict)?
            {
                valid += 1;
            }
        }
    }

    Ok(valid)
}