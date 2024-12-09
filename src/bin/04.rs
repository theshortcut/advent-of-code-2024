use advent_of_code::{Dir, Matrix, CARDINALS, ORDINALS};
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = Matrix::<char>::from(input);
    let mut directions = Vec::from(CARDINALS);
    directions.extend(ORDINALS);
    let possible_start_cells = matrix
        .items()
        .filter(|cell| cell.val == 'X')
        .collect::<Vec<_>>();
    Some(
        directions
            .iter()
            .cartesian_product(possible_start_cells)
            .filter(|(dir, cell)| {
                let mut current_cell = *cell;
                "XMAS".chars().all(|c| {
                    let mut char_check = false;
                    if current_cell.val == c {
                        char_check = true;
                    }
                    if let Some(next_cell) = matrix.neighbor(&current_cell, dir) {
                        if current_cell == next_cell {
                            char_check = false;
                        }
                        current_cell = next_cell;
                    }
                    char_check
                })
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = Matrix::<char>::from(input);
    Some(
        matrix
            .items()
            .filter(|cell| {
                if cell.val != 'A' {
                    return false;
                };
                let mut diag1match = false;
                let mut diag2match = false;
                if let Some(nw) = matrix.neighbor(cell, &Dir::NW) {
                    if let Some(se) = matrix.neighbor(cell, &Dir::SE) {
                        diag1match =
                            nw.val == 'M' && se.val == 'S' || nw.val == 'S' && se.val == 'M';
                    }
                }
                if let Some(sw) = matrix.neighbor(cell, &Dir::SW) {
                    if let Some(ne) = matrix.neighbor(cell, &Dir::NE) {
                        diag2match =
                            sw.val == 'M' && ne.val == 'S' || sw.val == 'S' && ne.val == 'M';
                    }
                }
                diag1match && diag2match
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
