#![allow(dead_code)]

use std::collections::HashSet;

pub(crate) fn read_groups(data: &Vec<String>) -> Vec<HashSet<char>> {
    let mut groups: Vec<HashSet<char>> = Vec::new();
    for group in data.split(|s| s == "") {
        let mut group_set: HashSet<char> = HashSet::new();
        for response in group {
            for question in response.chars() {
                group_set.insert(question);
            }
        }
        groups.push(group_set);
    }

    // println!("groups := {:?}\nlen:={}", groups, groups.len());
    groups
}

pub(crate) fn solve_p1(groups: Vec<HashSet<char>>) -> usize {
    groups.iter().map(|g| g.len()).sum()
}
