use std::collections::{HashSet, VecDeque};

use array2d::Array2D;

advent_of_code::solution!(16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    RowUp,
    ColumnRight,
    RowDown,
    ColumnLeft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Field {
    Empty,
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
}

impl Field {
    fn from(input: char) -> Field {
        match input {
            '.' => Field::Empty,
            '-' => Field::Horizontal,
            '|' => Field::Vertical,
            '/' => Field::DiagonalUp,
            '\\' => Field::DiagonalDown,
            _ => panic!("invalid type"),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let start_light = ((0, 0), Direction::ColumnRight);

    let map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| line.chars().map(Field::from).collect::<Vec<Field>>())
            .collect::<Vec<Vec<Field>>>(),
    )
    .unwrap();

    let mut energized_tiles: HashSet<(isize, isize)> = HashSet::new();
    let mut light_beam_path: HashSet<((isize, isize), Direction)> = HashSet::new();

    let mut light_beam: VecDeque<((isize, isize), Direction)> = VecDeque::new();

    light_beam.push_back(start_light);

    let max_height = map.column_len() as isize;
    let max_width = map.row_len() as isize;

    while let Some((pos, dir)) = light_beam.pop_front() {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= max_height || pos.1 >= max_width {
            continue;
        }
        if !light_beam_path.contains(&(pos, dir)) {
            light_beam_path.insert((pos, dir));
            energized_tiles.insert(pos);
            match map.get(pos.0 as usize, pos.1 as usize) {
                Some(field) => match field {
                    Field::Empty => match dir {
                        Direction::RowUp => {
                            light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                        }
                        Direction::ColumnRight => {
                            light_beam.push_back(((pos.0, pos.1 + 1), Direction::ColumnRight))
                        }
                        Direction::RowDown => {
                            light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                        }
                        Direction::ColumnLeft => {
                            light_beam.push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft))
                        }
                    },
                    Field::Horizontal => match dir {
                        Direction::RowUp | Direction::RowDown => {
                            light_beam.push_back(((pos.0, pos.1 + 1), Direction::ColumnRight));
                            light_beam.push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft));
                        }
                        Direction::ColumnRight => {
                            light_beam.push_back(((pos.0, pos.1 + 1), Direction::ColumnRight))
                        }
                        Direction::ColumnLeft => {
                            light_beam.push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft))
                        }
                    },
                    Field::Vertical => match dir {
                        Direction::ColumnRight | Direction::ColumnLeft => {
                            light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp));
                            light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown));
                        }
                        Direction::RowUp => {
                            light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                        }
                        Direction::RowDown => {
                            light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                        }
                    },
                    Field::DiagonalUp => match dir {
                        Direction::RowUp => {
                            light_beam.push_back(((pos.0, pos.1 + 1), Direction::ColumnRight))
                        }
                        Direction::ColumnRight => {
                            light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                        }
                        Direction::RowDown => {
                            light_beam.push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft))
                        }
                        Direction::ColumnLeft => {
                            light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                        }
                    },
                    Field::DiagonalDown => match dir {
                        Direction::RowDown => {
                            light_beam.push_back(((pos.0, pos.1 + 1), Direction::ColumnRight))
                        }
                        Direction::ColumnLeft => {
                            light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                        }
                        Direction::RowUp => {
                            light_beam.push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft))
                        }
                        Direction::ColumnRight => {
                            light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                        }
                    },
                },
                None => (),
            }
        }
    }

    Some(energized_tiles.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| line.chars().map(Field::from).collect::<Vec<Field>>())
            .collect::<Vec<Vec<Field>>>(),
    )
    .unwrap();

    let max_height = map.column_len();
    let max_width = map.row_len();

    let mut start_positions: Vec<((isize, isize), Direction)> = Vec::new();

    for row in 0..max_height {
        start_positions.push(((row as isize, 0), Direction::ColumnRight));
        start_positions.push((
            (row as isize, max_width as isize - 1),
            Direction::ColumnLeft,
        ));
    }
    for col in 0..max_width {
        start_positions.push(((0, col as isize), Direction::RowDown));
        start_positions.push(((max_height as isize - 1, col as isize), Direction::RowUp));
    }
    let mut max_val = 0;

    for start_light in start_positions {
        let mut energized_tiles: HashSet<(isize, isize)> = HashSet::new();
        let mut light_beam_path: HashSet<((isize, isize), Direction)> = HashSet::new();

        let mut light_beam: VecDeque<((isize, isize), Direction)> = VecDeque::new();

        light_beam.push_back(start_light);

        while let Some((pos, dir)) = light_beam.pop_front() {
            if pos.0 < 0 || pos.1 < 0 || pos.0 >= max_height as isize || pos.1 >= max_width as isize {
                continue;
            }
            if !light_beam_path.contains(&(pos, dir)) {
                light_beam_path.insert((pos, dir));
                energized_tiles.insert(pos);
                match map.get(pos.0 as usize, pos.1 as usize) {
                    Some(field) => {
                        match field {
                            Field::Empty => match dir {
                                Direction::RowUp => {
                                    light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                                }
                                Direction::ColumnRight => light_beam
                                    .push_back(((pos.0, pos.1 + 1), Direction::ColumnRight)),
                                Direction::RowDown => {
                                    light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                                }
                                Direction::ColumnLeft => light_beam
                                    .push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft)),
                            },
                            Field::Horizontal => match dir {
                                Direction::RowUp | Direction::RowDown => {
                                    light_beam
                                        .push_back(((pos.0, pos.1 + 1), Direction::ColumnRight));
                                    light_beam
                                        .push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft));
                                }
                                Direction::ColumnRight => light_beam
                                    .push_back(((pos.0, pos.1 + 1), Direction::ColumnRight)),
                                Direction::ColumnLeft => light_beam
                                    .push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft)),
                            },
                            Field::Vertical => match dir {
                                Direction::ColumnRight | Direction::ColumnLeft => {
                                    light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp));
                                    light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown));
                                }
                                Direction::RowUp => {
                                    light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                                }
                                Direction::RowDown => {
                                    light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                                }
                            },
                            Field::DiagonalUp => match dir {
                                Direction::RowUp => light_beam
                                    .push_back(((pos.0, pos.1 + 1), Direction::ColumnRight)),
                                Direction::ColumnRight => {
                                    light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                                }
                                Direction::RowDown => light_beam
                                    .push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft)),
                                Direction::ColumnLeft => {
                                    light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                                }
                            },
                            Field::DiagonalDown => match dir {
                                Direction::RowDown => light_beam
                                    .push_back(((pos.0, pos.1 + 1), Direction::ColumnRight)),
                                Direction::ColumnLeft => {
                                    light_beam.push_back(((pos.0 - 1, pos.1), Direction::RowUp))
                                }
                                Direction::RowUp => light_beam
                                    .push_back(((pos.0, pos.1 - 1), Direction::ColumnLeft)),
                                Direction::ColumnRight => {
                                    light_beam.push_back(((pos.0 + 1, pos.1), Direction::RowDown))
                                }
                            },
                        }
                    }
                    None => (),
                }
            }
        }

        let val = energized_tiles.len();
        if val > max_val {
            max_val = val;
        }
    }

    Some(max_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
