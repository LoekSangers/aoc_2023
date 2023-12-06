advent_of_code::solution!(3);

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
        static ref RE_SYMBOL: Regex = Regex::new(r"([-!@$%^#&*()_+|~=`{}\[\]:;'<>?,\/])").unwrap();
    }

    let candidates: Vec<(isize, isize, isize, u32)> = input.lines().enumerate().flat_map(|(index, line)| {

        RE.find_iter(line).map(|x| {
            (index as isize, x.start() as isize, x.end() as isize, x.as_str().parse::<u32>().ok().unwrap())
        }).collect::<Vec<(isize, isize, isize, u32)>>()        
    }).collect();

    let symbols: Vec<(isize, isize)> = input.lines().enumerate().flat_map(|(index, line)| {

        RE_SYMBOL.find_iter(line).map(|x| {
            (index as isize, x.start() as isize)
        }).collect::<Vec<(isize, isize)>>()        
    }).collect();

    Some(candidates
        .iter()
        .filter_map(|(cand_line, cand_start, cand_end, cand_value)| {

            if symbols.iter().any(|(sym_line, sym_pos)| {
                (cand_line - sym_line)*(cand_line - sym_line) < 2 && 
                cand_start - 2 < *sym_pos && 
                *sym_pos < cand_end + 1
                }){
                    Some(cand_value)
                }else{
                    None
                }
    
        }).sum())

}

pub fn part_two(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
        static ref RE_GEAR: Regex = Regex::new(r"([*])").unwrap();
    }

    let candidates: Vec<(isize, isize, isize, u32)> = input.lines().enumerate().flat_map(|(index, line)| {

        RE.find_iter(line).map(|x| {
            (index as isize, x.start() as isize, x.end() as isize, x.as_str().parse::<u32>().ok().unwrap())
        }).collect::<Vec<(isize, isize, isize, u32)>>()        
    }).collect();

    let gears: Vec<(isize, isize)> = input.lines().enumerate().flat_map(|(index, line)| {

        RE_GEAR.find_iter(line).map(|x| {
            (index as isize, x.start() as isize)
        }).collect::<Vec<(isize, isize)>>()        
    }).collect();

    Some(gears
        .iter()
        .filter_map(|(sym_line, sym_pos)| {

            let values = candidates
                .iter()
                .filter_map(|(cand_line, cand_start, cand_end, cand_value)| {
                    if (cand_line - sym_line)*(cand_line - sym_line) < 2 && 
                    cand_start - 2 < *sym_pos && 
                    *sym_pos < cand_end + 1
                    {
                        Some(cand_value)
                    }else{
                        None
                    }
                }).collect::<Vec<&u32>>();
            
            if values.len() > 1 {
                Some(values[0] * values[1])
            }else{
                None
            }
    
        }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
