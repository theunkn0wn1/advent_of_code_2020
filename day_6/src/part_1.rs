use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;

pub(crate) fn read_groups(data: &Vec<String>) -> anyhow::Result<Vec<HashSet<char>>>
{
    let mut groups: Vec<HashSet<char>> = Vec::new();
        for group in data.split(|s| s == ""){
            let mut group_set: HashSet<char> = HashSet::new();
            for response in group{
                for question in response.chars(){
                    group_set.insert(question);
                }
            }
            groups.push(group_set);
        }

    println!("groups := {:?}\nlen:={}", groups, groups.len());
    Ok(groups)
}

pub(crate) fn solve_p1(groups: Vec<HashSet<char>>) -> anyhow::Result<usize>{
    Ok(
        groups.iter().map(|g| g.len()).sum()
    )
}