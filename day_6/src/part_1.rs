#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct GroupResponse {
    pub(crate) total_respondents: usize,
    pub(crate) answers: HashMap<char, usize>,
}
impl GroupResponse {
    pub fn new(total: usize) -> Self {
        Self {
            total_respondents: total,
            answers: HashMap::with_capacity(27),
        }
    }
}
pub(crate) fn read_groups(
    data: &Vec<String>,
) -> Vec<GroupResponse> {
    let mut groups: Vec<GroupResponse> = Vec::new();
    for group in data.split(|s| s == "") {
        let mut group_struct =
            GroupResponse::new(group.len());

        for response in group.iter() {
            for question in response.chars() {
                if let Some(existing) =
                    group_struct.answers.get_mut(&question)
                {
                    *existing += 1;
                } else {
                    group_struct.answers.insert(question, 1);
                }
            }
        }
        groups.push(group_struct);
    }

    println!(
        "groups := {:?}\nlen:={}",
        groups,
        groups.len()
    );
    groups
}

pub fn solve_p1(lines: &Vec<String>) -> usize {
    let groups = read_groups(lines);
    groups.iter().map(|g| g.answers.len()).sum()
}
