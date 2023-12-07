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

    let seeds = groups.next().unwrap()
        .split_ascii_whitespace()
        .filter_map(|seed| {
            if seed.as_bytes()[0].is_ascii_digit(){
                Some(seed.parse::<i64>().ok().unwrap())
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

    // Try with ranges as seed instead of numbers
    let result = seeds
    .collect::<Vec<i64>>()
    .chunks(2)
    .map(|seed| {

        let mut ranges = vec![(seed[0], seed[0] + seed[1])];
        for step in steps.clone().into_iter(){
            let mut mapped_ranges : Vec<(i64, i64)> = Vec::new();
            
            //transform the range into the new range(s) note that one range can become multiple
            for mapping in step {
                let mut unmapped_ranges = Vec::new();

                for range in ranges {
                    // m0---m1
                    //          r0---r1
                    if mapping.1 < range.0 {
                        unmapped_ranges.push(range);
                    }
                    
                    //          m0---m1
                    // r0---r1
                    else if range.1 < mapping.0 {
                        unmapped_ranges.push(range);
                    }
                    
                    
                    //     m0-----m1
                    // r0-----r1
                    else if range.0 <= mapping.0 && range.1 >= mapping.0 && range.1 <= mapping.1 {
                        if range.0 < mapping.0 {
                            unmapped_ranges.push((range.0, mapping.0-1));
                        }
                        mapped_ranges.push((mapping.0 - mapping.2, range.1 - mapping.2));
                    }

                    // m0-----m1
                    //    r0-----r1
                    else if mapping.0 <= range.0 && mapping.1 >= range.0 && mapping.1 <= range.1 {
                        if mapping.1 < range.1 {
                            unmapped_ranges.push((mapping.1+1, range.1));
                        }
                        mapped_ranges.push((range.0 - mapping.2, mapping.1 - mapping.2));
                    }

                    
                    
                    // m0---------m1
                    //    r0---r1
                    else if mapping.0 <= range.0 && mapping.1 >= range.1{
                        mapped_ranges.push((range.0 - mapping.2, range.1 - mapping.2));
                    }
                    
                    
                    //    m0--m1
                    // r0---------r1
                    else if range.0 <= mapping.0 && range.1 >= mapping.1{
                        mapped_ranges.push((mapping.0 - mapping.2, mapping.1 - mapping.2));
                        if range.0 < mapping.0{
                            unmapped_ranges.push((range.0, mapping.0-1));

                        }
                        if mapping.1 < range.1{
                            unmapped_ranges.push((mapping.1, range.1));

                        }
                    }

                    else {  
                        panic!("missed mapping");
                    }

                }
                ranges = unmapped_ranges
            }
            ranges.extend(mapped_ranges);
        }
        ranges
    }).map(|ranges|  ranges.iter().min().unwrap().0 as u32 ).collect::<Vec<u32>>();
    
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
