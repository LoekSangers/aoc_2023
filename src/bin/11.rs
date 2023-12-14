advent_of_code::solution!(11);

use std::cmp::{min, max};

use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE_GALAXY: Regex = Regex::new(r"#").unwrap();
    }
    let mut width:isize = 0;
    
    let mut expanded_rows: Vec<isize> = Vec::new();

    let galaxies: Vec<(isize, isize)> = input.lines().enumerate().flat_map(|(index, line)| {
        width = line.len() as isize;        
        let matches = RE_GALAXY.find_iter(line).map(|x| {
            (index as isize, x.start() as isize)
        }).collect::<Vec<(isize, isize)>>();
        if matches.len() == 0 {
            expanded_rows.push(index as isize);
        }
        matches
    }).collect();
    let galaxy_cols = galaxies.iter().map(|(_, y)| *y).collect::<Vec<isize>>();

    let expanded_cols = (0..width).filter(|c| !galaxy_cols.contains(c) ).collect::<Vec<isize>>();

    Some(
        galaxies
            .iter()
            .combinations(2)
            .map(|combination|{
                let left = combination.get(0).unwrap();
                let right = combination.get(1).unwrap();

                left.0.abs_diff(right.0) 
                    + left.1.abs_diff(right.1) 
                    + expanded_rows.iter().filter(|&&r| r > min(left.0, right.0) && r < max(left.0, right.0)).count()
                    + expanded_cols.iter().filter(|&&r| r > min(left.1, right.1) && r < max(left.1, right.1)).count()
                
            })
            .sum::<usize>() as u32
        )
}

pub fn part_two(input: &str) -> Option<u64> {
    lazy_static! {
        static ref RE_GALAXY: Regex = Regex::new(r"#").unwrap();
    }
    let mut width:isize = 0;
    
    let mut expanded_rows: Vec<isize> = Vec::new();

    let galaxies: Vec<(isize, isize)> = input.lines().enumerate().flat_map(|(index, line)| {
        width = line.len() as isize;        
        let matches = RE_GALAXY.find_iter(line).map(|x| {
            (index as isize, x.start() as isize)
        }).collect::<Vec<(isize, isize)>>();
        if matches.len() == 0 {
            expanded_rows.push(index as isize);
        }
        matches
    }).collect();
    let galaxy_cols = galaxies.iter().map(|(_, y)| *y).collect::<Vec<isize>>();

    let expanded_cols = (0..width).filter(|c| !galaxy_cols.contains(c) ).collect::<Vec<isize>>();

    Some(
        galaxies
            .iter()
            .combinations(2)
            .map(|combination|{
                let left = combination.get(0).unwrap();
                let right = combination.get(1).unwrap();

                (
                    left.0.abs_diff(right.0) 
                        + left.1.abs_diff(right.1) 
                        + 999999 * expanded_rows.iter().filter(|&&r| r > min(left.0, right.0) && r < max(left.0, right.0)).count()
                        + 999999 * expanded_cols.iter().filter(|&&r| r > min(left.1, right.1) && r < max(left.1, right.1)).count()
                ) as u64
                
            })
            .sum::<u64>() as u64
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
