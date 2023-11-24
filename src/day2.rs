use itertools::Itertools;

type Dim = (u32, u32, u32);

fn parse_line(line: &str) -> Dim {
    line.split('x')
        .map(|num| num.trim().parse().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn generate(input: &str) -> Vec<Dim> {
    input.lines().map(parse_line).collect_vec()
}

fn sum_dim((l, w, h): &Dim) -> u32 {
    (2 * l * w) + (2 * w * h) + (2 * h * l)
}

fn two_mins((l, w, h): &Dim) -> (u32, u32) {
    let min1 = l.min(w).min(h);
    let min2 = if min1 == l {
        w.min(h)
    } else if min1 == w {
        l.min(h)
    } else {
        l.min(w)
    };
    (*min1, *min2)
}

fn extra(dim: &Dim) -> u32 {
    let (min1, min2) = two_mins(dim);
    min1 * min2
}

fn sum_paper(dim: &Dim) -> u32 {
    sum_dim(dim) + extra(dim)
}

pub fn part_1(input: &[Dim]) -> u32 {
    input.iter().map(sum_paper).sum()
}

fn ribbon(dim: &Dim) -> u32 {
    let (min1, min2) = two_mins(dim);
    (min1 * 2) + (min2 * 2) + (dim.0 * dim.1 * dim.2)
}

pub fn part_2(input: &[Dim]) -> u32 {
    input.iter().map(ribbon).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        assert_eq!(generate("2x3x4"), vec![(2, 3, 4)]);
        assert_eq!(generate("1x1x10"), vec![(1, 1, 10)]);
        assert_eq!(generate("2x3x4\n1x1x10"), vec![(2, 3, 4), (1, 1, 10)]);
    }

    #[test]
    fn test_sum_dim() {
        assert_eq!(sum_dim(&(2, 3, 4)), 52);
        assert_eq!(sum_dim(&(1, 1, 10)), 42);
    }

    #[test]
    fn test_extra() {
        assert_eq!(extra(&(2, 3, 4)), 6);
        assert_eq!(extra(&(1, 1, 10)), 1);
    }

    #[test]
    fn test_sum_paper() {
        assert_eq!(sum_paper(&(2, 3, 4)), 58);
        assert_eq!(sum_paper(&(1, 1, 10)), 43);
    }

    #[test]
    fn test_part_1() {
        let input = generate("2x3x4\n1x1x10");
        assert_eq!(part_1(&input), 58 + 43);
    }

    #[test]
    fn test_ribbon() {
        assert_eq!(ribbon(&(2, 3, 4)), 34);
        assert_eq!(ribbon(&(1, 1, 10)), 14);
    }

    #[test]
    fn test_part_2() {
        let input = generate("2x3x4\n1x1x10");
        assert_eq!(part_2(&input), 34 + 14);
    }
}
