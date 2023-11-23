use itertools::Itertools;

pub fn part_1(input: &str) -> isize {
    let counts = input.chars().counts();
    *counts.get(&'(').unwrap_or(&0) as isize - *counts.get(&')').unwrap_or(&0) as isize
}

pub fn part_2(input: &str) -> usize {
    let res = input.chars().enumerate().try_fold(0, |acc, (i, c)| {
        let acc = match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        };

        if acc == -1 {
            Err(i)
        } else {
            Ok(acc)
        }
    });
    match res {
        Ok(_) => panic!(),
        Err(index) => index + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&"(())"), 0);
        assert_eq!(part_1(&"()()"), 0);
        assert_eq!(part_1(&"((("), 3);
        assert_eq!(part_1(&"(()(()("), 3);
        assert_eq!(part_1(&"))((((("), 3);
        assert_eq!(part_1(&"())"), -1);
        assert_eq!(part_1(&"))("), -1);
        assert_eq!(part_1(&")))"), -3);
        assert_eq!(part_1(&")())())"), -3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&")"), 1);
        assert_eq!(part_2(&"()())"), 5);
    }
}
