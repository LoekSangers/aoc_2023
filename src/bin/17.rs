advent_of_code::solution!(17);

use std::collections::BinaryHeap;
use std::collections::HashSet;

use array2d::Array2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    RowUp,
    ColumnRight,
    RowDown,
    ColumnLeft,
}

#[derive(PartialEq, Eq, Hash)]
struct Step {
    position: (isize, isize),
    direction: Direction,
    steps: usize,
    score: u32,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Direction {
    fn allowed_directions(&self) -> [Direction; 3] {
        match self {
            Direction::RowUp => [
                Direction::ColumnRight,
                Direction::RowUp,
                Direction::ColumnLeft,
            ],
            Direction::ColumnRight => {
                [Direction::RowUp, Direction::RowDown, Direction::ColumnRight]
            }
            Direction::RowDown => [
                Direction::RowDown,
                Direction::ColumnRight,
                Direction::ColumnLeft,
            ],
            Direction::ColumnLeft => [Direction::RowUp, Direction::ColumnLeft, Direction::RowDown],
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
    )
    .unwrap();

    let mut visited: HashSet<((isize, isize), Direction, usize)> = HashSet::new();
    let mut to_visit: HashSet<((isize, isize), Direction, usize)> = HashSet::new();

    let mut to_explore: BinaryHeap<Step> = BinaryHeap::new();

    let left = Step {
        position: (0, 1),
        direction: Direction::ColumnRight,
        steps: 1,
        score: *map.get(0, 1).unwrap(),
    };
    let down = Step {
        position: (1, 0),
        direction: Direction::RowDown,
        steps: 1,
        score: *map.get(1, 0).unwrap(),
    };

    to_explore.push(left);
    to_explore.push(down);

    let max_height = map.column_len() as isize;
    let max_width = map.row_len() as isize;

    while let Some(step) = to_explore.pop() {
        if step.position.0 < 0
            || step.position.1 < 0
            || step.position.0 >= max_height
            || step.position.1 >= max_width
        {
            continue;
        }
        if step.position.0 == max_height - 1 && step.position.1 == max_width - 1 {
            return Some(step.score);
        }
        if !visited.contains(&(step.position, step.direction, step.steps)) {
            visited.insert((step.position, step.direction, step.steps));

            let dirs = step.direction.allowed_directions();
            for d in dirs {
                let mut taken_steps = 1;
                if d == step.direction {
                    taken_steps = step.steps + 1;
                    if taken_steps > 3 {
                        continue;
                    }
                }
                let new_pos = match d {
                    Direction::RowUp => (step.position.0 - 1, step.position.1),
                    Direction::ColumnRight => (step.position.0, step.position.1 + 1),
                    Direction::RowDown => (step.position.0 + 1, step.position.1),
                    Direction::ColumnLeft => (step.position.0, step.position.1 - 1),
                };
                let new_score = match map.get(new_pos.0 as usize, new_pos.1 as usize) {
                    Some(cost) => step.score + cost,
                    None => continue,
                };

                if !to_visit.contains(&(new_pos, d, taken_steps)) {
                    to_visit.insert((new_pos, d, taken_steps));
                    if !visited.contains(&(new_pos, d, taken_steps)) {
                        to_explore.push(Step {
                            position: new_pos,
                            direction: d,
                            steps: taken_steps,
                            score: new_score,
                        });
                    }
                }
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
    )
    .unwrap();

    let mut visited: HashSet<((isize, isize), Direction, usize)> = HashSet::new();
    let mut to_visit: HashSet<((isize, isize), Direction, usize)> = HashSet::new();

    let mut to_explore: BinaryHeap<Step> = BinaryHeap::new();

    //this is actually right xD
    let score_left = map.get(0, 1).unwrap()
        + map.get(0, 2).unwrap()
        + map.get(0, 3).unwrap()
        + map.get(0, 4).unwrap();
    let score_down = map.get(1, 0).unwrap()
        + map.get(2, 0).unwrap()
        + map.get(3, 0).unwrap()
        + map.get(4, 0).unwrap();

    let left = Step {
        position: (0, 4),
        direction: Direction::ColumnRight,
        steps: 4,
        score: score_left,
    };
    let down = Step {
        position: (4, 0),
        direction: Direction::RowDown,
        steps: 4,
        score: score_down,
    };

    to_explore.push(left);
    to_explore.push(down);

    let max_height = map.column_len() as isize;
    let max_width = map.row_len() as isize;

    while let Some(step) = to_explore.pop() {
        if step.position.0 < 0
            || step.position.1 < 0
            || step.position.0 >= max_height
            || step.position.1 >= max_width
        {
            continue;
        }
        if step.position.0 == max_height - 1 && step.position.1 == max_width - 1 {
            return Some(step.score);
        }
        if !visited.contains(&(step.position, step.direction, step.steps)) {
            visited.insert((step.position, step.direction, step.steps));

            let tmp_dirs = step.direction.allowed_directions();
            let tmp_dir = [step.direction; 1];

            let dirs = if step.steps >= 4 {
                tmp_dirs.as_slice()
            } else {
                tmp_dir.as_slice()
            };

            for &d in dirs {
                let mut taken_steps = 1;
                if d == step.direction {
                    taken_steps = step.steps + 1;
                    if taken_steps > 10 {
                        continue;
                    }
                }
                let new_pos = match d {
                    Direction::RowUp => (step.position.0 - 1, step.position.1),
                    Direction::ColumnRight => (step.position.0, step.position.1 + 1),
                    Direction::RowDown => (step.position.0 + 1, step.position.1),
                    Direction::ColumnLeft => (step.position.0, step.position.1 - 1),
                };
                let new_score = match map.get(new_pos.0 as usize, new_pos.1 as usize) {
                    Some(cost) => step.score + cost,
                    None => continue,
                };

                if !to_visit.contains(&(new_pos, d, taken_steps)) {
                    to_visit.insert((new_pos, d, taken_steps));
                    if !visited.contains(&(new_pos, d, taken_steps)) {
                        to_explore.push(Step {
                            position: new_pos,
                            direction: d,
                            steps: taken_steps,
                            score: new_score,
                        });
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
