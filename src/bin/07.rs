advent_of_code::solution!(7);

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(':');
            let result = parts.next().unwrap().trim().parse().unwrap();
            let nums = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (result, nums)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse(input);
    data.iter()
        .filter(|(result, numbers)| {
            numbers
                .iter()
                .fold(vec![], |mut acc, n| {
                    if acc.is_empty() {
                        acc.push(*n);
                    } else {
                        let mut new_acc = vec![];
                        for a in acc {
                            new_acc.push(a + n);
                            new_acc.push(a * n);
                        }
                        acc = new_acc;
                    }
                    acc
                })
                .iter()
                .any(|n| n == result)
        })
        .map(|(result, _)| *result)
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse(input);
    data.iter()
        .filter(|(result, numbers)| {
            numbers
                .iter()
                .fold(vec![], |mut acc, n| {
                    if acc.is_empty() {
                        acc.push(*n);
                    } else {
                        let mut new_acc = vec![];
                        for a in acc {
                            new_acc.push(a + n);
                            new_acc.push(a * n);
                            new_acc.push((a.to_string() + &n.to_string()).parse().unwrap());
                        }
                        acc = new_acc;
                    }
                    acc
                })
                .iter()
                .any(|n| n == result)
        })
        .map(|(result, _)| *result)
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
