use std::collections::HashSet;

use advent_of_code::{Dir, Matrix, Point};

advent_of_code::solution!(6);

fn turn_right(dir: &Dir) -> Dir {
    match dir {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
        _ => panic!("Invalid direction"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = Matrix::<char>::from(input);
    let mut visited = HashSet::<Point>::new();
    let mut current_dir = Dir::N;
    let mut current_cell = matrix.items().find(|c| c.val == '^').unwrap();
    visited.insert(current_cell.point);
    while let Some(cell) = matrix.neighbor(&current_cell, &current_dir) {
        match cell.val {
            '#' => {
                current_dir = turn_right(&current_dir);
            }
            _ => {
                current_cell = cell;
                visited.insert(cell.point);
            }
        }
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = Matrix::<char>::from(input);
    let possible_changes = matrix.items().filter(|c| c.val == '.').collect::<Vec<_>>();
    let mut loops = 0;
    for change in possible_changes {
        let mut visited = HashSet::<(Point, Dir)>::new();
        let mut current_dir = Dir::N;
        let mut current_cell = matrix.items().find(|c| c.val == '^').unwrap();
        visited.insert((current_cell.point, current_dir));
        while let Some(cell) = matrix.neighbor(&current_cell, &current_dir) {
            if visited.contains(&(cell.point, current_dir)) {
                loops += 1;
                break;
            }
            match cell.val {
                '#' => {
                    current_dir = turn_right(&current_dir);
                }
                _ if cell.point == change.point => {
                    current_dir = turn_right(&current_dir);
                }
                _ => {
                    current_cell = cell;
                    visited.insert((cell.point, current_dir));
                }
            }
        }
    }
    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
