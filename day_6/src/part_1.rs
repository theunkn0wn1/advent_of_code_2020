use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;

pub(crate) fn read_groups(data: &Vec<String>) -> anyhow::Result<()>
{
    let groups: Vec<HashSet<char>> = data
        .iter().filter(|s| *s != "")
        .map(|v| HashSet::from_iter(v.chars())).collect_vec();
    println!("groups := {:?}", groups);

    Ok(())
}
