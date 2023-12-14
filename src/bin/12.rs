advent_of_code::solution!(12);
use memoize::memoize;

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input.lines().map(|line| {     
            let mut parts = line.split_ascii_whitespace();
            let conditions = parts.next().unwrap().to_owned();
                
            let summary = parts.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

            let res = find_options(conditions, summary, false);

            // println!("{}", res);

            res
        })
        .sum::<u64>()
    )
}

#[memoize]
fn find_options(condition: String, summary: Vec<usize>, inside_group: bool) -> u64{
    // println!("{} - {}", condition, summary.len());
    let mut tmp_summary = summary.clone();
    if inside_group && !tmp_summary.is_empty() {
        tmp_summary[0] -= 1;
    }

    if tmp_summary.is_empty() {
        if condition.contains('#'){
            0
        }else{
            1
        }
    }else if condition.is_empty(){
        if tmp_summary[0] == 0 && tmp_summary.len() == 1{
            1
        }else{
            0
        }
    }else if tmp_summary[0] == 0 {
        match condition.get(..1){
            Some("?") | Some(".") => {
                tmp_summary.remove(0);
                find_options(
                condition.get(1..).unwrap().to_owned(),
                tmp_summary, 
                 false)
                },
            _ => 0
        }        
    }else if inside_group{
        match condition.get(..1){
            Some("?") | Some("#") => {
                find_options(
                condition.get(1..).unwrap().to_owned(),
                tmp_summary, 
                 true)},
            _ => 0
        } 
    }
    else{
        match condition.get(..1){
            Some("#") => {
                find_options(
                    condition.get(1..).unwrap().to_owned(),
                    tmp_summary, 
                    true)},
            Some(".") => {
                find_options(
                    condition.get(1..).unwrap().to_owned(),
                    tmp_summary, 
                    false)},
            _ => {
                find_options(
                    condition.get(1..).unwrap().to_owned(),
                    tmp_summary.clone(), 
                    false)
                +
                find_options(
                    condition.get(1..).unwrap().to_owned(),
                    tmp_summary, 
                    true)
            },
        }         
    }  
}

pub fn part_two(input: &str) -> Option<u64> {

    Some(
        input.lines().map(|line| {     
            let mut parts = line.split_ascii_whitespace();
            let base_condition = parts.next().unwrap();
            let mut conditions = base_condition.to_owned();
            for _ in 0..4{
                conditions.push('?');
                conditions.push_str(base_condition);
            }

            let base_summary = parts.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let summary = base_summary.iter().cycle().take(base_summary.len() * 5).map(|&x| x).collect::<Vec<usize>>();


            let res = find_options(conditions, summary, false);

            // println!("{}", res);

            res
        })
        .sum::<u64>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}