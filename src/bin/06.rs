use std::iter::zip;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let time = lines[0].split_ascii_whitespace().skip(1).map(|s| s.parse::<f64>().ok().unwrap());
    let dist = lines[1].split_ascii_whitespace().skip(1).map(|s| s.parse::<f64>().ok().unwrap());

    Some(zip(time, dist).map(|(t, d)| {
        1 + (((t+(t*t-4_f64*(d+1_f64)).powf(0.5_f64))/2_f64 as f64 ).floor() - ((t-(t*t-4_f64*(d+1_f64)).powf(0.5_f64))/2_f64 as f64).ceil()) as u32
    }).reduce(|a, b| a * b).unwrap())

// with Wolfram alpha:
// 1 + floor((t+sqrt(t^2-4*(d+1)))/2) - ceil((t-sqrt(t^2-4*(d+1)))/2) for t = 30 and d = 200

}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let t = lines[0].split(':').skip(1).next().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<f64>().ok().unwrap();
    let d = lines[1].split(':').skip(1).next().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<f64>().ok().unwrap();
    Some(
        1 + (((t+(t*t-4_f64*(d+1_f64)).powf(0.5_f64))/2_f64 as f64 ).floor() - ((t-(t*t-4_f64*(d+1_f64)).powf(0.5_f64))/2_f64 as f64).ceil()) as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
