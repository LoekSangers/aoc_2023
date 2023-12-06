advent_of_code::solution!(2);
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
        static ref RE_RED: Regex = Regex::new(r"(\d+) red").unwrap();
        static ref RE_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
    }

    input.lines().filter_map(|line|{
        let parts: Vec<&str> = line.split(':').collect();
        let grabs: Vec<&str> = parts[1].split(';').collect();

        let (red, green, blue) = grabs.iter().map(|grab| {
            let green = match RE_GREEN.captures(grab) {
                Some(caps) => {
                    caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
                }
                None => 0,
            };
            let red = match RE_RED.captures(grab) {
                Some(caps) => {
                    caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
                }
                None => 0,
            };
            let blue = match RE_BLUE.captures(grab) {
                Some(caps) => {
                    caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
                }
                None => 0,
            };
            (red, green, blue)
        }).reduce(|(r1, g1, b1), (r2, g2, b2)| 
            (cmp::max(r1, r2),cmp::max(g1, g2),cmp::max(b1, b2))).unwrap();
            
        if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
            None
        }else{
            Some(parts[0].split_ascii_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().ok())
        }

    }).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
        static ref RE_RED: Regex = Regex::new(r"(\d+) red").unwrap();
        static ref RE_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
    }

    input.lines().map(|line|{
        let parts: Vec<&str> = line.split(':').collect();
        let grabs: Vec<&str> = parts[1].split(';').collect();

        let (red, green, blue) = grabs.iter().map(|grab| {
            let green = match RE_GREEN.captures(grab) {
                Some(caps) => {
                    caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
                }
                None => 0,
            };
            let red = match RE_RED.captures(grab) {
                Some(caps) => {
                    caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
                }
                None => 0,
            };
            let blue = match RE_BLUE.captures(grab) {
                Some(caps) => {
                    caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
                }
                None => 0,
            };
            (red, green, blue)
        }).reduce(|(r1, g1, b1), (r2, g2, b2)| 
            (cmp::max(r1, r2),cmp::max(g1, g2),cmp::max(b1, b2))).unwrap();
            
        Some(red * green * blue)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
