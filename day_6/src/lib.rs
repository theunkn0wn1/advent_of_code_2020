#[cfg(test)]
mod tests {
    #[test]
    fn test_read_groups() {
        let lines = day_1::read_lines("sample.txt")
            .expect("failed to read lines")
            .collect::<Result<Vec<_>, _>>()
            .expect("bad read.");
        crate::part_1::read_groups(&lines).expect("read groups failed.");
    }
}
mod part_1;
