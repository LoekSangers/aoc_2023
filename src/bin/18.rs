advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {

    let path = input.lines().fold((vec![(0,0)], (0,0)), |mut acc, elem| {        
        let mut parts = elem.split_ascii_whitespace();

        let direction = parts.next().unwrap();
        let step = parts.next().unwrap().parse::<usize>().unwrap() as isize;

        let pos = acc.1;

        let new_pos = match direction {
            "U" => (pos.0 - step, pos.1),
            "R" => (pos.0 , pos.1 + step),
            "D" => (pos.0 + step, pos.1),
            "L" => (pos.0 , pos.1 - step),
            _ => panic!("unknown")
        };
        acc.0.push(new_pos);  
        (acc.0, new_pos)
    }).0;

    let border = path.windows(2).map(|corners| {
        corners[0].0.abs_diff(corners[1].0) + corners[0].1.abs_diff(corners[1].1)
    }).sum::<usize>();

    let concave_area = path.windows(2).map(|corners| {
        corners[0].0*corners[1].1 - corners[0].1*corners[1].0
    }).sum::<isize>();
    

    Some(1 + ((border + concave_area.abs() as usize) / 2) as usize)    
}

pub fn part_two(input: &str) -> Option<usize> {

    let path = input.lines().fold((vec![(1, 1)], (1, 1)), |mut acc, elem| {        
        let parts = elem.split_ascii_whitespace().skip(2).next().unwrap().replace(&['(','#',')'],"");

        let direction = &parts[5..];
        let step = isize::from_str_radix(&parts[..5], 16).unwrap() as isize;

        let pos = acc.1;

        let new_pos = match direction {
            "3" => (pos.0 - step, pos.1),
            "0" => (pos.0 , pos.1 + step),
            "1" => (pos.0 + step, pos.1),
            "2" => (pos.0 , pos.1 - step),
            _ => panic!("unknown")
        };
        acc.0.push(new_pos);  
        (acc.0, new_pos)
    }).0;
    
    let border = path.windows(2).map(|corners| {
        corners[0].0.abs_diff(corners[1].0) + corners[0].1.abs_diff(corners[1].1)
    }).sum::<usize>();

    let concave_area = path.windows(2).map(|corners| {
        corners[0].0*corners[1].1 - corners[0].1*corners[1].0
    }).sum::<isize>();
    

    Some(1 + ((border + concave_area.abs() as usize) / 2) as usize)    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
