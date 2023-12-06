use lazy_static::lazy_static;
use regex::{Regex, Match};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d)").unwrap();
    }

    Some(input.lines()
        .map(|line| {
            let matches = RE.find_iter(line).map(|x| x.as_str().to_owned()).collect::<Vec<String>>();
            (matches.first().unwrap().to_owned() + matches.last().unwrap()).parse::<u32>().unwrap()
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    }

    Some(input.lines()
        .map(|line| {
            let mut matches:Vec<Match> = Vec::new();

            let mut m = RE.find_at(line, 0).unwrap();
            matches.push(m);
            loop {
                let tmp = RE.find_at(line, m.start()+1);
                
                match tmp {
                    Some(x) => {m = x; matches.push(m)},
                    None => break
                }
            }

            let results = matches.iter().map(|x| {
                let found = x.as_str();

                if found.as_bytes()[0].is_ascii_digit() {
                    found.to_owned()
                }
                else{
                    match found {
                        "one" => "1".to_owned(),
                        "two" => "2".to_owned(),
                        "three" => "3".to_owned(),
                        "four" => "4".to_owned(),
                        "five" => "5".to_owned(),
                        "six" => "6".to_owned(),
                        "seven" => "7".to_owned(),
                        "eight" => "8".to_owned(),
                        "nine" => "9".to_owned(),
                        _ => panic!()
                    }
                }
            }
        ).collect::<Vec<String>>();

        let res = (results.first().unwrap().to_owned() + results.last().unwrap()).parse::<u32>().unwrap();
        res
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
