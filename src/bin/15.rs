use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|hashable| {
                hashable.as_bytes().iter().fold(0_u32, |acc, elem| {
                    ((((acc + *elem as u32) % 256) * 17) % 256) as u32
                })
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: HashMap<u8, HashMap<String, u32>> = HashMap::new();
    for i in 0..u8::MAX{
        boxes.insert(i,  HashMap::new());
    }
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .for_each(|info| {
                let label = &info[..2];
                let correct_box = label.as_bytes().iter().fold(0_u8, |acc, elem| {
                    ((((acc as u32 + *elem as u32) % 256) * 17) % 256) as u8
                });

                let operation = &info[2..3];
                
                match operation {
                    "=" => {
                        let focal = info[3..4].parse::<u32>().unwrap();
                        boxes.get_mut(&correct_box).unwrap().insert(label.to_string(),focal);
                    },
                    "-" => {
                        boxes.get_mut(&correct_box).unwrap().remove(label);
                    },
                    _ => panic!("invalid operation")
                }
            });
    
    let mut res: u32 = 0;
    for i in 0..u8::MAX{
        //Order is wrong
        for elem in boxes.get(&i).unwrap().values().enumerate(){
            println!("{} * {} * {}", i+1, elem.0 + 1, elem.1);
            res += (1+i as u32) * elem.0 as u32 + 1 * elem.1;
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
