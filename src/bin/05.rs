advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut groups = input.split("\n\n");

    let seeds = groups.next().unwrap()
        .split_ascii_whitespace()
        .filter_map(|seed| {
            if seed.as_bytes()[0].is_ascii_digit(){
                Some(seed.parse::<u32>().ok().unwrap())
            }else{
                None
            }
        });

    let steps = groups.map(|step|{
        step.lines().skip(1).map(|line|{
            let args: Vec<u64> = line.split_ascii_whitespace().map(|n| n.parse::<u64>().ok().unwrap()).collect();
        
            (args[1] as i64, (args[1] + args[2]) as i64, args[1] as i64 - args[0] as i64)
        })
    });

    let result = seeds
        .map(|seed| {
            let mut tmp_val = seed as i64;
            for step in steps.clone().into_iter(){
                let diff = match step.clone().find(|(start, end, _)| {
                    start <= &tmp_val && &tmp_val <= end
                }) {
                    Some(x) => x.2,
                    None => 0
                };
                tmp_val -= diff;
            }
            tmp_val
        }).map(|x| x as u32).collect::<Vec<u32>>();

    // println!("{:#?}", result);

    Some(*result.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut groups = input.split("\n\n");

    let parsing = groups.next().unwrap()
        .split_ascii_whitespace()
        .filter_map(|seed| {
            if seed.as_bytes()[0].is_ascii_digit(){
                Some(seed.parse::<i64>().ok().unwrap())
            }else{
                None
            }
        }).collect::<Vec<i64>>();
    let seeds = parsing
        .chunks(2)
        .flat_map(|slice| {
            (slice[0]..slice[0]+slice[1]).collect::<Vec<i64>>()
        });

    println!("{:#?}", seeds.clone().collect::<Vec<i64>>().len());

    let steps = groups.map(|step|{
        let mut parts = step.lines().skip(1).map(|line|{
            let args: Vec<u64> = line.split_ascii_whitespace().map(|n| n.parse::<u64>().ok().unwrap()).collect();
        
            (args[0] as i64, (args[0] + args[2]) as i64, args[0] as i64 - args[1] as i64)
        }).collect::<Vec<(i64, i64, i64)>>();

        parts.sort();

        parts
    }).collect::<Vec<Vec<(i64, i64, i64)>>>();

    // Try with ranges as seed instead of numbers
    let result = seeds
    .collect::<Vec<i64>>()
    .chunks(2)
    .map(|seed| {

        let mut ranges = vec![(seed[0], seed[0] + seed[1])];
        for step in steps.clone().into_iter(){
            let mut new_ranges : Vec<(i64, i64)> = Vec::new();
            
            //transform the range into the new range(s) note that one range can become multiple
            for range in ranges {
                let diff = match step.clone().find(|(start, end, _)| {
                    start <= &tmp_val && &tmp_val <= end
                }) {
                    Some(x) => x.2,
                    None => 0
                };
                tmp_val -= diff;

            }

            ranges = new_ranges;
        }
        ranges
    }).map(|range|  range.iter().min().unwrap().0 as u32 ).collect::<Vec<u32>>();
    
    Some(*result.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
