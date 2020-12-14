use std::collections::HashSet;

pub(crate) fn read_groups(data: &Vec<String>) -> anyhow::Result<()> {
    let groups: Vec<Vec<String>> = data.split(|s| s == "\n\n").map(|s| s.to_owned()).collect();
    println!("groups := {:?}", groups);

    Ok(())
}
