use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

fn parse_dir(input: char) -> Dir {
    match input {
        '^' => Dir::North,
        'v' => Dir::South,
        '>' => Dir::East,
        '<' => Dir::West,
        _ => unreachable!(),
    }
}

pub fn generate(input: &str) -> Vec<Dir> {
    input.chars().map(parse_dir).collect()
}

pub fn part_1(input: &[Dir]) -> usize {
    let mut map: HashMap<(isize, isize), bool> = HashMap::new();
    map.insert((0, 0), true);
    input.iter().fold((0, 0), |pos, dir| {
        let new_pos = match dir {
            Dir::North => (pos.0, pos.1 + 1),
            Dir::South => (pos.0, pos.1 - 1),
            Dir::East => (pos.0 + 1, pos.1),
            Dir::West => (pos.0 - 1, pos.1),
        };
        map.insert(new_pos, true);
        new_pos
    });
    map.values().filter(|&val| *val).count()
}

pub fn part_2(input: &[Dir]) -> usize {
    let santa = input.iter().step_by(2);
    let robot = input.iter().skip(1).step_by(2);

    let mut map: HashMap<(isize, isize), bool> = HashMap::new();
    map.insert((0, 0), true);

    santa.fold((0, 0), |pos: (isize, isize), dir: &Dir| {
        let new_pos = match dir {
            Dir::North => (pos.0, pos.1 + 1),
            Dir::South => (pos.0, pos.1 - 1),
            Dir::East => (pos.0 + 1, pos.1),
            Dir::West => (pos.0 - 1, pos.1),
        };
        map.insert(new_pos, true);
        new_pos
    });
    robot.fold((0, 0), |pos: (isize, isize), dir: &Dir| {
        let new_pos = match dir {
            Dir::North => (pos.0, pos.1 + 1),
            Dir::South => (pos.0, pos.1 - 1),
            Dir::East => (pos.0 + 1, pos.1),
            Dir::West => (pos.0 - 1, pos.1),
        };
        map.insert(new_pos, true);
        new_pos
    });
    map.values().filter(|&val| *val).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        assert_eq!(generate(">"), vec![Dir::East]);
        assert_eq!(
            generate("^>v<"),
            vec![Dir::North, Dir::East, Dir::South, Dir::West]
        );
    }

    #[test]
    fn test_part_1() {
        let input1 = generate(">");
        assert_eq!(part_1(&input1), 2);

        let input2 = generate("^>v<");
        assert_eq!(part_1(&input2), 4);

        let input3 = generate("^v^v^v^v^v");
        assert_eq!(part_1(&input3), 2);
    }

    #[test]
    fn test_part_2() {
        let input1 = generate("^v");
        assert_eq!(part_2(&input1), 3);

        let input2 = generate("^>v<");
        assert_eq!(part_2(&input2), 3);

        let input3 = generate("^v^v^v^v^v");
        assert_eq!(part_2(&input3), 11);
    }
}
