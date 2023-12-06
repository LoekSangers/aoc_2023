advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(|line|{
        let parts: Vec<&str> = line.split(':').collect();
        let numbers: Vec<&str> = parts[1].split('|').collect();
        let winning_numbers: Vec<u32> = numbers[0].split(' ').filter_map(|s| if !s.is_empty() {Some(s.parse::<u32>().unwrap())}else{None}).collect();
        let card_numbers = numbers[1].split(' ').filter_map(|s| if !s.is_empty() {Some(s.parse::<u32>().unwrap())}else{None});


        let matches = card_numbers.fold(0, |acc, n| {
            if winning_numbers.contains(&n) {
                acc + 1
            }else{
                acc
            }});
        

        if matches == 0 {
            None
        }else{            
            Some(2_u32.pow(matches-1))
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counts: Vec<u32> = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate(){
        let parts: Vec<&str> = line.split(':').collect();
        let numbers: Vec<&str> = parts[1].split('|').collect();
        let winning_numbers: Vec<u32> = numbers[0].split(' ').filter_map(|s| if !s.is_empty() {Some(s.parse::<u32>().unwrap())}else{None}).collect();
        let card_numbers = numbers[1].split(' ').filter_map(|s| if !s.is_empty() {Some(s.parse::<u32>().unwrap())}else{None});

        let matches = card_numbers.fold(0, |acc, n| {
            if winning_numbers.contains(&n) {
                acc + 1
            }else{
                acc
            }});
        
        for i in index+1..index+matches+1{
            counts[i] += counts[index]
        }        
    }

    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
