advent_of_code::solution!(5);

#[derive(Debug)]
struct OrderRule(u32, u32);

fn parse(input: &str) -> (Vec<OrderRule>, Vec<Vec<u32>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rules: Vec<OrderRule> = rules_str
        .lines()
        .map(|l| {
            let (before, after) = l.split_once('|').unwrap();
            OrderRule(before.parse().unwrap(), after.parse().unwrap())
        })
        .collect();
    let updates: Vec<Vec<u32>> = updates_str
        .lines()
        .map(|l| l.split(',').map(|c| c.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    Some(
        updates
            .iter()
            .filter(|u| {
                let mut last = u[0];
                for &n in u.iter().skip(1) {
                    if !rules.iter().any(|r| r.0 == last && r.1 == n) {
                        return false;
                    }
                    last = n;
                }
                true
            })
            .map(|u| u[u.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    Some(
        updates
            .iter()
            .filter(|u| {
                let mut last = u[0];
                for &n in u.iter().skip(1) {
                    if !rules.iter().any(|r| r.0 == last && r.1 == n) {
                        return true;
                    }
                    last = n;
                }
                false
            })
            .map(|u| {
                let mut u = u.clone();
                u.sort_unstable_by(|a, b| {
                    if rules.iter().any(|r| r.0 == *a && r.1 == *b) {
                        std::cmp::Ordering::Less
                    } else if rules.iter().any(|r| r.0 == *a && r.1 == *b) {
                        return std::cmp::Ordering::Greater;
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                u
            })
            .map(|u| u[u.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
