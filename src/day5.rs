use itertools::Itertools;

fn nice(input: &str) -> bool {
    // at least three vowels
    static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let vowel_count = input.chars().filter(|c| VOWELS.contains(c)).count();
    if vowel_count < 3 {
        return false;
    }

    // at least one letter that appears twice in a row
    let appears_twice = input
        .chars()
        .collect_vec()
        .windows(2)
        .any(|pair| pair[0] == pair[1]);
    if !appears_twice {
        return false;
    }

    // does not contain the strings ab, cd, pq, or xy
    static NO_CONTAINS: [&str; 4] = ["ab", "cd", "pq", "xy"];
    let contains = NO_CONTAINS.iter().any(|&set| input.contains(set));
    if contains {
        return false;
    }

    true
}

pub fn part_1(input: &str) -> usize {
    input.lines().filter(|&line| nice(line)).count()
}

fn nice_2(input: &str) -> bool {
    // contains at least one letter which repeats with exactly one letter between them
    if !input
        .chars()
        .collect_vec()
        .windows(3)
        .any(|set| set[0] == set[2])
    {
        return false;
    }

    // contains a pair of any two letters that appears at least twice in the string without overlapping

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice() {
        assert!(nice("ugknbfddgicrmopn"));
        assert!(nice("aaa"));
        assert!(!nice("jchzalrnumimnmhp"));
        assert!(!nice("haegwjzuvuyypxyu"));
        assert!(!nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_nice_2() {
        assert!(nice_2("qjhvhtzxzqqjkmpb"));
        assert!(nice_2("xxyxx"));
        assert!(!nice_2("uurcxstgmygtbstg"));
        assert!(!nice_2("ieodomkazucvgmuy"));
    }
}
