use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|c| {
                let (_, [a, b]) = c.extract();
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                a * b
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?:mul\((\d{1,3},\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    let mut mul_enabled = true;
    Some(
        re.captures_iter(input)
            .map(|c| {
                let (haystack, [c]) = c.extract();
                if haystack.starts_with("mul") && mul_enabled {
                    c.split(',').map(|x| x.parse::<u32>().unwrap()).product()
                } else if haystack == "do()" {
                    mul_enabled = true;
                    0
                } else if haystack == "don't()" {
                    mul_enabled = false;
                    0
                } else {
                    0
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
