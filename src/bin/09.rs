advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut differences: Vec<Vec<i64>> = Vec::new();
                differences.push(line.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>());

                loop {
                    let mut all_zeroes = true;
                    let next_line = differences.last().unwrap().windows(2).map(|a| {
                        let diff = a[1] - a[0]; 
                        all_zeroes &= diff == 0; 
                        diff
                    }).collect::<Vec<i64>>();
                    differences.push(next_line);

                    if all_zeroes{
                        break
                    }
                }
                for i in (1..differences.len()).rev() {
                    let next_val = differences[i-1][differences[i-1].len()-1] + differences[i][differences[i].len()-1];
                    differences[i-1].push(next_val);
                }
                // println!("{:#?}", differences);

                differences[0][differences[0].len()-1]
            })
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut differences: Vec<Vec<i64>> = Vec::new();
                differences.push(line.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>());

                loop {
                    let mut all_zeroes = true;
                    let next_line = differences.last().unwrap().windows(2).map(|a| {
                        let diff = a[1] - a[0]; 
                        all_zeroes &= diff == 0; 
                        diff
                    }).collect::<Vec<i64>>();
                    differences.push(next_line);

                    if all_zeroes{
                        break
                    }
                }
                for i in (1..differences.len()).rev() {
                    let next_val = differences[i-1][0] - differences[i][differences[i].len()-1];
                    differences[i-1].push(next_val);
                }
                // println!("{:#?}", differences);

                differences[0][differences[0].len()-1]
            })
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
