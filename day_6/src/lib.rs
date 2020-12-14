#[cfg(test)]
mod tests {

    #[test]
    fn test_read_groups() {
        let lines = day_1::read_lines("sample.txt")
            .expect("failed to read lines")
            .collect::<Result<Vec<_>, _>>()
            .expect("bad read.");
        let groups = crate::part_1::read_groups(&lines);
        assert_eq!(groups.len(), 5)
    }
}
mod part_1;
