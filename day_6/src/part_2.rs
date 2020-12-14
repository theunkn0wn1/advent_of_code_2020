use crate::part_1::read_groups;

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_groups() {
        let lines = day_1::read_lines("sample.txt")
            .expect("failed to read lines")
            .collect::<Result<Vec<_>, _>>()
            .expect("bad read.");
        let valid_groups = crate::part_2::solve_p2(&lines);
        assert_eq!(valid_groups, 6)
    }
}

pub fn solve_p2(lines: &Vec<String>) -> usize {
    let groups = read_groups(lines);
    let mut sum = 0;
    for group in groups {
        for (_, total_respondents) in group.answers {
            if total_respondents == group.total_respondents
            {
                sum += 1;
            }
        }
    }
    sum
}
