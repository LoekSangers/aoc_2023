advent_of_code::solution!(13);
use array2d::Array2D;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Ground {
    Ash,
    Rock,
}

impl Ground {
    fn from(input: char) -> Ground {
        match input {
            '.'  => Ground::Ash,
            '#'  => Ground::Rock,
            _      => panic!("invalid type"),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut mapping = input
            .lines()
            .fold((Vec::<Array2D<Ground>>::new(), Vec::<Vec::<Ground>>::new()),|mut acc, line| {
                if line.len() > 0 {
                    acc.1.push(line.chars().map(Ground::from).collect());

                    acc
                }else{
                    acc.0.push(Array2D::from_rows(&acc.1).unwrap());
                    (acc.0, Vec::<Vec::<Ground>>::new())
                }
            });
    mapping.0.push(Array2D::from_rows(&mapping.1).unwrap());
    let maps = mapping.0;

    let candidate_rows = maps.iter().filter_map(|map| {
        let rows = map.as_rows();
        let enumerated = rows.iter().enumerate().collect::<Vec<_>>();
        let candidates = enumerated.windows(2).filter_map(|window|{
            if window[0].1 == window[1].1 {
                Some(window[0].0)
            }else{
                None
            }
        });

        let res = candidates.filter_map(|candidate|{
            let mut left = rows[..candidate+1].to_owned();
            let right = rows[candidate+1..].to_owned();

            left.reverse();

            if left.iter().zip(&right).all(|test| test.0 == test.1){
                Some(candidate)
            }else{
                None
            }

        }).collect::<Vec<_>>();

        match res.first(){
            Some(&x) => Some(x),
            None => None
        }
    }).collect::<Vec<_>>();

    // println!("{:#?}", candidate_rows);

    let candidate_cols = maps.iter().filter_map(|map| {
        let columns = map.as_columns();
        let enumerated = columns.iter().enumerate().collect::<Vec<_>>();
        let candidates = enumerated.windows(2).filter_map(|window|{
            if window[0].1 == window[1].1 {
                Some(window[0].0)
            }else{
                None
            }
        });

        let res = candidates.filter_map(|candidate|{
            let mut left = columns[..candidate+1].to_owned();
            let right = columns[candidate+1..].to_owned();

            left.reverse();

            if left.iter().zip(&right).all(|test| test.0 == test.1){
                Some(candidate)
            }else{
                None
            }

        }).collect::<Vec<_>>();

        match res.first(){
            Some(&x) => Some(x),
            None => None
        }
    }).collect::<Vec<_>>();

    // println!("{:#?}", candidate_cols);

    Some(
        candidate_rows.iter().map(|&x| x+1).sum::<usize>() * 100 
            + candidate_cols.iter().map(|&x| x+1).sum::<usize>()
    )
}

fn dist(vec1: &Vec<Ground>, vec2: &Vec<Ground>) -> usize {
    vec1.iter().zip(vec2).map(|combi| {
        if combi.0 == combi.1{
            0
        }else{
            1
        }
    }).sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut mapping = input
            .lines()
            .fold((Vec::<Array2D<Ground>>::new(), Vec::<Vec::<Ground>>::new()),|mut acc, line| {
                if line.len() > 0 {
                    acc.1.push(line.chars().map(Ground::from).collect());

                    acc
                }else{
                    acc.0.push(Array2D::from_rows(&acc.1).unwrap());
                    (acc.0, Vec::<Vec::<Ground>>::new())
                }
            });
    mapping.0.push(Array2D::from_rows(&mapping.1).unwrap());
    let maps = mapping.0;

    let candidate_rows = maps.iter().filter_map(|map| {
        let rows = map.as_rows();
        let enumerated = rows.iter().enumerate().collect::<Vec<_>>();
        let candidates = enumerated.windows(2).filter_map(|window|{
            if dist(window[0].1, window[1].1) <= 1 {
                Some(window[0].0)
            }else{
                None
            }
        });

        let res = candidates.filter_map(|candidate|{
            let mut left = rows[..candidate+1].to_owned();
            let right = rows[candidate+1..].to_owned();

            left.reverse();

            if left.iter().zip(&right).map(|test| {
                dist(test.0, test.1)
            }).sum::<usize>() == 1{
                Some(candidate)
            }else{
                None
            }

        }).collect::<Vec<_>>();

        match res.first(){
            Some(&x) => Some(x),
            None => None
        }
    }).collect::<Vec<_>>();

    // println!("{:#?}", candidate_rows);

    let candidate_cols = maps.iter().filter_map(|map| {
        let columns = map.as_columns();
        let enumerated = columns.iter().enumerate().collect::<Vec<_>>();
        let candidates = enumerated.windows(2).filter_map(|window|{
            if dist(window[0].1, window[1].1) <= 1 {
                Some(window[0].0)
            }else{
                None
            }
        });

        let res = candidates.filter_map(|candidate|{
            let mut left = columns[..candidate+1].to_owned();
            let right = columns[candidate+1..].to_owned();

            left.reverse();

            if left.iter().zip(&right).map(|test| {
                dist(test.0, test.1)
            }).sum::<usize>() == 1{
                Some(candidate)
            }else{
                None
            }

        }).collect::<Vec<_>>();

        match res.first(){
            Some(&x) => Some(x),
            None => None
        }
    }).collect::<Vec<_>>();

    // println!("{:#?}", candidate_cols);

    Some(
        candidate_rows.iter().map(|&x| x+1).sum::<usize>() * 100 
            + candidate_cols.iter().map(|&x| x+1).sum::<usize>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
