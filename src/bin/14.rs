advent_of_code::solution!(14);
use array2d::Array2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
    Empty,
}

impl Rock {
    fn from(input: char) -> Rock {
        match input {
            '.'  => Rock::Empty,
            'O'  => Rock::Round,
            '#'  => Rock::Square,
            _    => panic!("invalid type"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Array2D::from_rows(
            &input
            .lines()
            .map(|line|{
                line.chars().map(Rock::from).collect::<Vec<Rock>>()
            }).collect::<Vec<Vec<Rock>>>()
        )
        .unwrap()
        .as_columns()
        .iter()
        .map(|col| {
            let length = col.len() as u32;
            col.iter().enumerate().fold((Vec::<u32>::new(), 0), |mut acc, elem| {
                match elem.1 {
                    Rock::Empty => (),
                    Rock::Round => {
                        acc.0.push(length - acc.1);
                        acc.1 += 1;
                    },
                    Rock::Square => {
                        acc.1 = elem.0 as u32 + 1;
                    }
                }
                acc
            } 
            ).0.iter().sum::<u32>()
        }).sum()

    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Array2D::from_rows(
        &input
        .lines()
        .map(|line|{
            line.chars().map(Rock::from).collect::<Vec<Rock>>()
        }).collect::<Vec<Vec<Rock>>>()
    )
    .unwrap();

    let mut maps: Vec<Array2D<Rock>> = Vec::new();

    loop{
        let mut swaps: Vec<((usize, usize), (usize, usize))> = Vec::new();
        //North
        {
        swaps.append(map
            .columns_iter()
            .enumerate()
            .flat_map(|(index, col)|{
                col.enumerate().fold((Vec::<((usize, usize), (usize, usize))>::new(), 0), |mut acc, elem| {
                    match elem.1 {
                        Rock::Empty => (),
                        Rock::Round => {
                            acc.0.push(((acc.1, index), (elem.0, index)));
                            acc.1 += 1;
                        },
                        Rock::Square => {
                            acc.1 = elem.0 + 1;
                        }
                    }
                    acc
                }).0
            })
            .collect::<Vec::<((usize, usize), (usize, usize))>>()
            .to_owned()
            .as_mut());
        }
        for swap in &swaps {
            let tmp = map[swap.0].clone();
            map[swap.0] = map[swap.1];
            map[swap.1] = tmp;
        }
        swaps.clear();

        //West
        {
            swaps.append(map
                .rows_iter()
                .enumerate()
                .flat_map(|(index, row)|{
                    row.enumerate().fold((Vec::<((usize, usize), (usize, usize))>::new(), 0), |mut acc, elem| {
                        match elem.1 {
                            Rock::Empty => (),
                            Rock::Round => {
                                acc.0.push(((index, acc.1), (index, elem.0)));
                                acc.1 += 1;
                            },
                            Rock::Square => {
                                acc.1 = elem.0 + 1;
                            }
                        }
                        acc
                    }).0
                })
                .collect::<Vec::<((usize, usize), (usize, usize))>>()
                .to_owned()
                .as_mut());
        }
        for swap in &swaps {
            let tmp = map[swap.0].clone();
            map[swap.0] = map[swap.1];
            map[swap.1] = tmp;
        }
        swaps.clear();

        //South
        {
            swaps.append(map
                .columns_iter()
                .enumerate()
                .flat_map(|(index, col)|{
                    let enumerated_col = col.enumerate().collect::<Vec<(usize, &Rock)>>();
                    let length = enumerated_col.len();
                    enumerated_col.iter().rev().fold((Vec::<((usize, usize), (usize, usize))>::new(), length-1), |mut acc, elem| {
                        match elem.1 {
                            Rock::Empty => (),
                            Rock::Round => {
                                acc.0.push(((acc.1, index), (elem.0, index)));
                                acc.1 -= 1;
                            },
                            Rock::Square => {
                                if elem.0 > 0{
                                    acc.1 = elem.0 - 1;
                                }
                            }
                        }
                        acc
                    }).0
                })
                .collect::<Vec::<((usize, usize), (usize, usize))>>()
                .to_owned()
                .as_mut());
            }
            for swap in &swaps {
                let tmp = map[swap.0].clone();
                map[swap.0] = map[swap.1];
                map[swap.1] = tmp;
            }
            swaps.clear();

        //East
        {
            swaps.append(map
                .rows_iter()
                .enumerate()
                .flat_map(|(index, row)|{
                    let enumerated_row = row.enumerate().collect::<Vec<(usize, &Rock)>>();
                    let length = enumerated_row.len();
                    enumerated_row.iter().rev().fold((Vec::<((usize, usize), (usize, usize))>::new(), length-1), |mut acc, elem| {
                        match elem.1 {
                            Rock::Empty => (),
                            Rock::Round => {
                                acc.0.push(((index, acc.1), (index, elem.0)));
                                acc.1 -= 1;
                            },
                            Rock::Square => {
                                if elem.0 > 0{
                                    acc.1 = elem.0 - 1;
                                }
                            }
                        }
                        acc
                    }).0
                })
                .collect::<Vec::<((usize, usize), (usize, usize))>>()
                .to_owned()
                .as_mut());
        }
        for swap in &swaps {
            let tmp = map[swap.0].clone();
            map[swap.0] = map[swap.1];
            map[swap.1] = tmp;
        }
        swaps.clear();

        if maps.contains(&map){
            break;
        }else{
            maps.push(map.clone());
        }
    }
    let first_index = maps.iter().position(|m|m == &map).unwrap();
    let diff = maps.len() - first_index;

    map = maps.get(first_index + (1000000000 - first_index)%diff - 1).unwrap().clone();

    Some(
        map
        .as_columns()
        .iter()
        .map(|col| {
            let length = col.len() as u32;
            col.iter().enumerate().fold(Vec::<u32>::new(), |mut acc, elem| {
                match elem.1 {
                    Rock::Empty => (),
                    Rock::Round => {
                        acc.push(length - elem.0 as u32);
                    },
                    Rock::Square => ()
                }
                acc
            } 
            ).iter().sum::<u32>()
        }).sum()

    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
