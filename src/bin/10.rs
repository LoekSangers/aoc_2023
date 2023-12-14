use std::collections::HashMap;
use std::collections::VecDeque;
use std::str;
use geo::Within;
use geo_types::point;
use geo_types::{LineString, Polygon};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(PartialEq, Debug)]
enum Direction {
    North, 
    East,   
    South,
    West,
    Exit,
}

impl Direction {
    fn oppostite(&self) -> Direction {
        match self{
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Exit => Direction::Exit,
        }
    }

    fn translation(&self) -> (isize, isize){
        match self{
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::Exit => (0, 0)
        }
    }
}

fn next_direction(cell_type: &[u8], enter_direction: Direction) -> Direction{
    match cell_type {
        b"|" => {
            match enter_direction {
                Direction::South => Direction::North,
                Direction::North => Direction::South,
                _ => Direction::Exit
            }
        },
        b"-" => {
            match enter_direction {
                Direction::West => Direction::East,
                Direction::East => Direction::West,
                _ => Direction::Exit
            }
        },
        b"L" => {
            match enter_direction {
                Direction::East => Direction::North,
                Direction::North => Direction::East,
                _ => Direction::Exit
            }
        },
        b"J" => {
            match enter_direction {
                Direction::West => Direction::North,
                Direction::North => Direction::West,
                _ => Direction::Exit
            }
        },
        b"7" => {
            match enter_direction {
                Direction::South => Direction::West,
                Direction::West => Direction::South,
                _ => Direction::Exit
            }
        },
        b"F" => {
            match enter_direction {
                Direction::South => Direction::East,
                Direction::East => Direction::South,
                _ => Direction::Exit
            }
        },
        b"." | b"\n" => Direction::Exit,
        _ => panic!("invalid unknown char")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.find('\n').unwrap() + 2;
    let mut field = ".".repeat(width-1).to_owned();
    field.push_str("\n.");
    field.push_str(&input.replace('\n', "\n."));
    field.push_str(".\n");
    field.push_str(&(".".repeat(width-1)));
    field.push_str(&("\n"));

    let map = field.as_bytes();

    let start_index = field.find('S').unwrap();
    let start: (isize, isize) = ((start_index % width) as isize, (start_index / width) as isize); 
        
    let mut visited: HashMap<(isize, isize), u32> = HashMap::new();
    let mut to_visit: VecDeque<(isize, isize, u32, Direction)> = VecDeque::new();

    visited.insert((start.0, start.1), 0);
    for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
        let translation = dir.translation();
        let new_loc = (start.0 + translation.0, start.1 + translation.1, 1, dir.oppostite()); 
        if next_direction(&[map[(new_loc.0 + width as isize * new_loc.1) as usize]], dir.oppostite()) != Direction::Exit {

            visited.insert((new_loc.0, new_loc.1), new_loc.2);
            to_visit.push_back(new_loc);
        }
    }
    let mut end = 0;

    while let Some(cur_node) = to_visit.pop_front() {
        let next_dir = next_direction(&[map[(cur_node.0 + width as isize * cur_node.1) as usize]], cur_node.3);
        if next_dir == Direction::Exit {
            continue;
        }        
        let translation = next_dir.translation();
        let new_loc = (cur_node.0 + translation.0, cur_node.1 + translation.1, cur_node.2+1, next_dir.oppostite());
        if !visited.keys().contains(&(new_loc.0, new_loc.1)){
            visited.insert((new_loc.0, new_loc.1), new_loc.2);
            to_visit.push_back(new_loc); 
        }else{
            end = visited.get(&(new_loc.0, new_loc.1)).unwrap().to_owned();
            break;
        }
        // println!("{:#?}", visited);
    }

    Some(end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.find('\n').unwrap() + 2;
    let mut field = ".".repeat(width-1).to_owned();
    field.push_str("\n.");
    field.push_str(&input.replace('\n', "\n."));
    field.push_str("\n");
    field.push_str(&(".".repeat(width-1)));
    field.push_str(&("\n"));

    let map = field.as_bytes();

    let start_index = field.find('S').unwrap();
    let start: (isize, isize) = ((start_index % width) as isize, (start_index / width) as isize); 
        
    let mut visited: HashMap<(isize, isize), ((isize, isize), u32)> = HashMap::new();
    let mut to_visit: VecDeque<(isize, isize, u32, Direction)> = VecDeque::new();

    visited.insert((start.0, start.1), ((0,0), 0));
    for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
        let translation = dir.translation();
        let new_loc = (start.0 + translation.0, start.1 + translation.1, 1, dir.oppostite()); 
        if next_direction(&[map[(new_loc.0 + width as isize * new_loc.1) as usize]], dir.oppostite()) != Direction::Exit {

            visited.insert((new_loc.0, new_loc.1), ((start.0, start.1),new_loc.2));
            to_visit.push_back(new_loc);
        }
    }

    let mut path: Vec<(isize, isize)> = Vec::new();

    while let Some(cur_node) = to_visit.pop_front() {
        let next_dir = next_direction(&[map[(cur_node.0 + width as isize * cur_node.1) as usize]], cur_node.3);
        if next_dir == Direction::Exit {
            continue;
        }        
        let translation = next_dir.translation();
        let new_loc = (cur_node.0 + translation.0, cur_node.1 + translation.1, cur_node.2+1, next_dir.oppostite());
        if !visited.keys().contains(&(new_loc.0, new_loc.1)){
            visited.insert((new_loc.0, new_loc.1), ((cur_node.0, cur_node.1),new_loc.2));
            to_visit.push_back(new_loc); 
        }else{
            //construct the path
            path.push((new_loc.0, new_loc.1));
            let mut cur_path = (new_loc.0, new_loc.1);
            while let Some(val) = visited.get(&cur_path) {
                if val.0 == (0, 0){
                    break;
                }
                cur_path = val.0;
                path.push(cur_path)
            }
            let mut other_part: Vec<(isize, isize)> = Vec::new();
            cur_path = (cur_node.0, cur_node.1);
            other_part.push(cur_path);
            while let Some(val) = visited.get(&cur_path) {
                if val.0 == (0, 0){
                    break;
                }
                cur_path = val.0;
                other_part.push(cur_path)
            }
            other_part.reverse();
            path.extend(other_part);
            path.push((new_loc.0, new_loc.1));
            break;
        }
    }
    let poly = Polygon::new(
        LineString::from(path),
        vec![],
    );
    let mut count = 0;

    for x in 0..width as isize {
        for y in 0..(map.len()/width) as isize {
            let point = point!(x: x, y: y);
            if point.is_within(&poly){
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
