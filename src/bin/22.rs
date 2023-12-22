use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap};

use array2d::Array2D;

advent_of_code::solution!(22);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Brick {
    id: usize,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    min_z: usize,
    z_dif: usize,
}
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min_z.cmp(&self.min_z)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.min_z.partial_cmp(&self.min_z)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let bricks = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let binding = line.replace(&[',', '~'], " ");
            let mut parts = binding
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap());

            let x1 = parts.next().unwrap();
            let y1 = parts.next().unwrap();
            let z1 = parts.next().unwrap();

            let x2 = parts.next().unwrap();
            let y2 = parts.next().unwrap();
            let z2 = parts.next().unwrap();

            Brick {
                id: i,
                min_x: min(x1, x2),
                max_x: max(x1, x2),
                min_y: min(y1, y2),
                max_y: max(y1, y2),
                min_z: min(z1, z2),
                z_dif: z1.abs_diff(z2),
            }
        })
        .collect::<Vec<Brick>>();

    let mut brick_heights: HashMap<usize, usize> = HashMap::new();

    let mut map = Array2D::from_row_major(&vec![0; 100], 10, 10).unwrap();
    let mut sorted_bricks: BinaryHeap<Brick> = BinaryHeap::from(bricks.clone());
    while let Some(brick) = sorted_bricks.pop() {
        let mut max_map_z: usize = 0;
        for x in brick.min_x..=brick.max_x {
            for y in brick.min_y..=brick.max_y {
                let &pos_z = map.get(x, y).unwrap();
                if pos_z > max_map_z {
                    max_map_z = pos_z;
                }
            }
        }
        for x in brick.min_x..=brick.max_x {
            for y in brick.min_y..=brick.max_y {
                let pos_z = map.get_mut(x, y).unwrap();
                *pos_z = max_map_z + 1 + brick.z_dif;
            }
        }
        brick_heights.insert(brick.id, max_map_z + 1);
    }

    let mut disintegratable = 0;

    for disintegratable_brick in bricks.clone() {
        let mut disintegrate = true;
        let mut disintegratable_map = Array2D::from_row_major(&vec![0; 100], 10, 10).unwrap();
        let mut sorted_disintegratable_bricks: BinaryHeap<Brick> = BinaryHeap::from(bricks.clone());
        while let Some(brick) = sorted_disintegratable_bricks.pop() {
            if brick.id == disintegratable_brick.id {
                continue
            }
            let mut max_map_z: usize = 0;
            for x in brick.min_x..=brick.max_x {
                for y in brick.min_y..=brick.max_y {
                    let &pos_z = disintegratable_map.get(x, y).unwrap();
                    if pos_z > max_map_z {
                        max_map_z = pos_z;
                    }
                }
            }
            
            for x in brick.min_x..=brick.max_x {
                for y in brick.min_y..=brick.max_y {
                    let pos_z = disintegratable_map.get_mut(x, y).unwrap();
                    *pos_z = max_map_z + 1 + brick.z_dif;
                }
            }
            if brick_heights.get(&brick.id) != Some(&(max_map_z + 1)){
                disintegrate = false;
                break;
            }
        }
        if disintegrate {
            disintegratable +=1;
        }
    }

    Some(disintegratable)
}

pub fn part_two(input: &str) -> Option<usize> {
    let bricks = input
    .lines()
    .enumerate()
    .map(|(i, line)| {
        let binding = line.replace(&[',', '~'], " ");
        let mut parts = binding
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap());

        let x1 = parts.next().unwrap();
        let y1 = parts.next().unwrap();
        let z1 = parts.next().unwrap();

        let x2 = parts.next().unwrap();
        let y2 = parts.next().unwrap();
        let z2 = parts.next().unwrap();

        Brick {
            id: i,
            min_x: min(x1, x2),
            max_x: max(x1, x2),
            min_y: min(y1, y2),
            max_y: max(y1, y2),
            min_z: min(z1, z2),
            z_dif: z1.abs_diff(z2),
        }
    })
    .collect::<Vec<Brick>>();

let mut brick_heights: HashMap<usize, usize> = HashMap::new();

let mut map = Array2D::from_row_major(&vec![0; 100], 10, 10).unwrap();
let mut sorted_bricks: BinaryHeap<Brick> = BinaryHeap::from(bricks.clone());
while let Some(brick) = sorted_bricks.pop() {
    let mut max_map_z: usize = 0;
    for x in brick.min_x..=brick.max_x {
        for y in brick.min_y..=brick.max_y {
            let &pos_z = map.get(x, y).unwrap();
            if pos_z > max_map_z {
                max_map_z = pos_z;
            }
        }
    }
    for x in brick.min_x..=brick.max_x {
        for y in brick.min_y..=brick.max_y {
            let pos_z = map.get_mut(x, y).unwrap();
            *pos_z = max_map_z + 1 + brick.z_dif;
        }
    }
    brick_heights.insert(brick.id, max_map_z + 1);
}

let mut falling = 0;

for disintegratable_brick in bricks.clone() {
    let mut disintegratable_map = Array2D::from_row_major(&vec![0; 100], 10, 10).unwrap();
    let mut sorted_disintegratable_bricks: BinaryHeap<Brick> = BinaryHeap::from(bricks.clone());
    while let Some(brick) = sorted_disintegratable_bricks.pop() {
        if brick.id == disintegratable_brick.id {
            continue
        }
        let mut max_map_z: usize = 0;
        for x in brick.min_x..=brick.max_x {
            for y in brick.min_y..=brick.max_y {
                let &pos_z = disintegratable_map.get(x, y).unwrap();
                if pos_z > max_map_z {
                    max_map_z = pos_z;
                }
            }
        }
        
        for x in brick.min_x..=brick.max_x {
            for y in brick.min_y..=brick.max_y {
                let pos_z = disintegratable_map.get_mut(x, y).unwrap();
                *pos_z = max_map_z + 1 + brick.z_dif;
            }
        }
        if brick_heights.get(&brick.id) != Some(&(max_map_z + 1)){
            falling += 1;
        }
    }
}

Some(falling)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
