use std::collections::{HashMap, HashSet};

type GroupType = Vec<HashMap<char, HashSet<usize>>>;
pub fn solve_p2(lines: &Vec<String>) -> usize {
    0
}
pub(crate) fn read_groups(data: &Vec<String>) -> GroupType{
    let mut groups: GroupType = Vec::new();
    for group in data.split(|s| s == "") {
        // declare a hashmap to store this group's responses
        // and reserve 27 items, as we know there are only 27 questions.
        let mut group_dict: HashMap<char, HashSet<usize>> = HashMap::with_capacity(27);
        // for each person who answered (i), take their (response)
        for (i, response) in group.iter().enumerate() {
            // and split it up into the questions they answered in the affirmative
            for question in response.chars() {
                // If the answer is already in the dict
                if let Some(set) = group_dict.get_mut(&question) {
                    // just insert the current respondent.
                    set.insert(i);
                } else {
                    // otherwise we will need to create it.
                    // We know how many potential respondents in the group there is,
                    // Therefore we can reserve that many potential items up-front.
                    let mut set = HashSet::with_capacity(group.len());
                    set.insert(i);
                    group_dict.insert(question, set);
                }
            }
        }
        groups.push(group_dict);

    }

    // println!("groups := {:?}\nlen:={}", groups, groups.len());
    groups
}
