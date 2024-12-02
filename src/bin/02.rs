advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn check_row(row: &[u32]) -> bool {
    let delta_range = 1..=3;
    let mut direction = None;
    let mut prev: Option<u32> = None;
    let mut is_safe = true;
    for level in row.iter() {
        match (prev, &direction) {
            (Some(prev), None) if &prev > level => {
                direction = Some(Direction::Descending);
                let diff = prev.abs_diff(*level);
                if !delta_range.contains(&diff) {
                    is_safe = false;
                }
            }
            (Some(prev), None) if &prev < level => {
                direction = Some(Direction::Ascending);
                let diff = prev.abs_diff(*level);
                if !delta_range.contains(&diff) {
                    is_safe = false;
                }
            }
            (Some(prev), None) if &prev == level => {
                is_safe = false;
            }
            (Some(prev), Some(dir)) => {
                match dir {
                    Direction::Ascending => {
                        if &prev >= level {
                            is_safe = false;
                        }
                    }
                    Direction::Descending => {
                        if &prev <= level {
                            is_safe = false;
                        }
                    }
                };
                let diff = prev.abs_diff(*level);
                if !delta_range.contains(&diff) {
                    is_safe = false;
                }
            }
            _ => (),
        }
        prev = Some(*level);
    }
    is_safe
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = parse(input);
    Some(data.iter().filter(|r| check_row(r)).count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let mut count = 0;
    for row in data.iter() {
        for (i, _) in row.iter().enumerate() {
            let mut row = row.clone();
            row.remove(i);
            if check_row(&row) {
                count += 1;
                break;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
