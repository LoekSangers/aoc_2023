advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {

// with Wolfram alpha:
// 1 + floor((t+sqrt(t^2-4*(d+1)))/2) - ceil((t-sqrt(t^2-4*(d+1)))/2) for t = 30 and d = 200

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
