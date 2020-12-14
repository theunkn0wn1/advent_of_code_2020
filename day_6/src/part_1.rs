use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;

pub(crate) fn read_groups(data: &Vec<String>) -> anyhow::Result<()>
{
    let groups: Vec<HashSet<&String>> = data.split(|s| s == "")
        .map(
            |group| HashSet::from_iter(group)
        ).collect_vec();

    println!("groups := {:?}\nlen:={}", groups, groups.len());
    Ok(())
}
