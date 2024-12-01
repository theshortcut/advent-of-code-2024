advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut bits = line.split("   ");
        list1.push(bits.next().unwrap().parse::<u32>().unwrap());
        list2.push(bits.next().unwrap().parse::<u32>().unwrap());
    }
    list1.sort_unstable();
    list2.sort_unstable();
    let mut sum = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        sum += a.abs_diff(*b);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut bits = line.split("   ");
        list1.push(bits.next().unwrap().parse::<u32>().unwrap());
        list2.push(bits.next().unwrap().parse::<u32>().unwrap());
    }
    let mut score = 0;
    // loop through list 1 and count how many times each number appears in list 2
    for num in list1.iter() {
        let mut count = 0;
        for num2 in list2.iter() {
            if num == num2 {
                count += 1;
            }
        }
        score += num * count;
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
