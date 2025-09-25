#[cfg(test)]
mod tests {
    use common::Track;

    fn build_track_from_csv(line: &str) -> Track {
        let mut parts = line.split(',');

        Track::new(
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
            parts.next().unwrap_or_default().parse::<u32>().ok(),
            parts.next().unwrap_or_default().parse::<u32>().ok(),
            parts.next().unwrap_or_default().parse::<u32>().ok(),
            None,
        )
    }

    fn compare<F>(file: &str, assert_fn: F)
    where
        F: Fn(&Track, &Track),
    {
        let lines: Vec<&str> = file.lines().collect();

        for pair in lines.windows(2).skip(1) {
            if pair.len() == 2 {
                assert_fn(
                    &build_track_from_csv(pair[0]),
                    &build_track_from_csv(pair[1]),
                );
            }
        }
    }

    #[test]
    fn compare_equal_from_csv() {
        const DATA: &str = include_str!("fixtures/tracks_equal.csv");
        compare(DATA, |a, b| assert_eq!(a, b));
    }

    #[test]
    fn compare_diff_from_csv() {
        const DATA: &str = include_str!("fixtures/tracks_different.csv");
        compare(DATA, |a, b| assert_ne!(a, b));
    }
}
