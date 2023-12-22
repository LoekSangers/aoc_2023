use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    let mut start = (-1, -1);
    let gardens =
        input
            .lines()
            .enumerate()
            .fold(HashSet::<(isize, isize)>::new(), |mut acc, (row, line)| {
                for (col, c) in line.chars().enumerate() {
                    match c {
                        '.' => {
                            acc.insert((row as isize, col as isize));
                        }
                        'S' => {
                            start = (row as isize, col as isize);
                            acc.insert((row as isize, col as isize));
                        }
                        _ => (),
                    }
                }

                acc
            });
    let mut reachable_gardens: HashSet<(isize, isize)> = HashSet::new();

    reachable_gardens.insert(start);

    let mut to_explore: VecDeque<((isize, isize), isize)> = VecDeque::new();
    to_explore.push_back((start, 0));

    while let Some((current, distance)) = to_explore.pop_front() {
        let new_distance = distance + 1;
        if new_distance > 64 {
            break;
        }

        let targets = [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ];

        for target in targets {
            if gardens.contains(&target) && !reachable_gardens.contains(&target) {
                if new_distance % 2 == 0 {
                    reachable_gardens.insert(target);
                }
                to_explore.push_back((target, new_distance));
            }
        }
    }

    Some(reachable_gardens.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let steps = 26501365;
    let dim = input.lines().count();

    let mut start = (-1, -1);

    let gardens =
        input
            .lines()
            .enumerate()
            .fold(HashSet::<(isize, isize)>::new(), |mut acc, (row, line)| {
                for (col, c) in line.chars().enumerate() {
                    match c {
                        '.' => {
                            acc.insert((row as isize, col as isize));
                        }
                        'S' => {
                            start = (row as isize, col as isize);
                            acc.insert((row as isize, col as isize));
                        }
                        _ => (),
                    }
                }

                acc
            });

    let mut to_explore = VecDeque::new();
    to_explore.push_back((start, 0));
    
    let mut reachable_gardens_even: HashSet<(isize, isize)> = HashSet::new();
    let mut reachable_gardens_odd: HashSet<(isize, isize)> = HashSet::new();
    let mut reachable_gardens_even_corner: HashSet<(isize, isize)> = HashSet::new();
    let mut reachable_gardens_odd_corner: HashSet<(isize, isize)> = HashSet::new();

    reachable_gardens_even.insert(start);

    while let Some((current, distance)) = to_explore.pop_front() {
        let new_distance = distance + 1;
        if !gardens.contains(&current) {
            continue;
        }

        let targets = [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ];

        for target in targets {
            if gardens.contains(&target) {
                if new_distance % 2 == 0 && !reachable_gardens_even.contains(&target) {
                    reachable_gardens_even.insert(target);
                    to_explore.push_back((target, new_distance));
                    if new_distance > 65 {
                        reachable_gardens_even_corner.insert(target);
                    }
                }else if new_distance % 2 == 1 && !reachable_gardens_odd.contains(&target) {
                    reachable_gardens_odd.insert(target);
                    to_explore.push_back((target, new_distance));
                    if new_distance > 65 {
                        reachable_gardens_odd_corner.insert(target);
                    }
                }
            }
        }
    }

    let odd = reachable_gardens_odd.len();
    let even = reachable_gardens_even.len();
    let odd_corners = reachable_gardens_odd_corner.len();
    let even_corners = reachable_gardens_even_corner.len();

    let count = steps / dim;

    let total_odd = odd * (count + 1) * (count + 1); // count all odd squares including outer ring
    let total_even = even * (count * count); // count all even  squares

    let total_odd_corners = odd_corners * (count + 1); // we over counted the corners of odd squares in the outer ring
    let total_even_corners = count * even_corners; // we did not count the even corners 'outside' the outer ring

    Some(total_odd + total_even - total_odd_corners + total_even_corners)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(625382480005896));
    }
}
