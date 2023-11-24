fn compute(secret: &str, answer: &u64, take: usize) -> bool {
    let input = format!("{}{}", secret, answer.to_string());
    let digest = md5::compute(&input);
    let digest = format!("{:x}", digest).chars().take(take).all(|c| c == '0');
    digest
}

pub fn part_1(secret: &str) -> u64 {
    (1..u64::MAX)
        .find(|answer| compute(secret, answer, 5))
        .unwrap()
}

pub fn part_2(secret: &str) -> u64 {
    (1..u64::MAX)
        .find(|answer| compute(secret, answer, 6))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("abcdef"), 609043);
        assert_eq!(part_1("pqrstuv"), 1048970);
    }

    #[test]
    fn test_compute() {
        assert!(compute("abcdef", &609043, 5));
        assert!(!compute("abcdef", &609040, 5));
        assert!(compute("pqrstuv", &1048970, 5));
    }
}
