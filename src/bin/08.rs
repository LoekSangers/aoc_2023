use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars();

    let network = lines.skip(1).fold(HashMap::<String, (String, String)>::new(),|mut acc, line| {
        let binding = line.replace(&['(', ')', '=', ','], "");
        let mut mapping = binding.split_ascii_whitespace();

        acc.insert(mapping.next().unwrap().to_string(), (mapping.next().unwrap().to_string(), mapping.next().unwrap().to_string()));

        acc
    });

    let mut counter = 0;
    let mut current_node = "AAA".to_string();
    let end_node = "ZZZ".to_string();

    let mut resetting_instructions = instructions.clone();

    loop {
        if current_node == end_node {
            break;
        }

        match resetting_instructions.next() {
            Some('L') => {
                current_node = network.get(&current_node).unwrap().0.to_string();
                
                counter += 1;
            },
            Some('R') => {
                current_node = network.get(&current_node).unwrap().1.to_string();
                
                counter += 1;
            },
            _ => {
                resetting_instructions = instructions.clone();
            }
        }
    }

    Some(counter)
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars();

    
    let mut current_nodes: Vec<String> = Vec::new();

    let network = lines.skip(1).fold(HashMap::<String, (String, String)>::new(),|mut acc, line| {
        let binding = line.replace(&['(', ')', '=', ','], "");
        let mut mapping = binding.split_ascii_whitespace();

        let key = mapping.next().unwrap().to_string(); 
        if key.chars().nth(2) == Some('A'){
            current_nodes.push(key.clone());
        }

        acc.insert(key, (mapping.next().unwrap().to_string(), mapping.next().unwrap().to_string()));

        acc
    });

    let mut counts: Vec<u64> = Vec::new();

    for node in current_nodes {
        let mut counter = 0;
        let mut current_node = node;
        let mut resetting_instructions = instructions.clone();
        loop {
            if current_node.chars().nth(2) == Some('Z'){
                break;
            }

            match resetting_instructions.next() {
                Some('L') => {
                    current_node = network.get(&current_node).unwrap().0.to_string();
                    
                    counter += 1;
                },
                Some('R') => {
                    current_node = network.get(&current_node).unwrap().1.to_string();
                    
                    counter += 1;
                },
                _ => {
                    resetting_instructions = instructions.clone();
                }
            }
        }

        counts.push(counter);
    }

    // println!("{:#?}", counts);

    Some(lcm(&counts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
