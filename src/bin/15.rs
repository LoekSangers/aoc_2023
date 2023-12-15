use std::collections::HashMap;

use itertools::Itertools;

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
    let mut boxes: HashMap<u8, HashMap<String, (u32, u32)>> = HashMap::new();
    let mut order_index = 0_u32;
    for i in 0..u8::MAX{
        boxes.insert(i,  HashMap::new());
    }
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .for_each(|info| {
                let label = info[..2].to_string();
                let correct_box = label.as_bytes().iter().fold(0_u8, |acc, elem| {
                    ((((acc as u32 + *elem as u32) % 256) * 17) % 256) as u8
                });

                let operation = &info[2..3];//keys have different sizes needs fixing still
                
                match operation {
                    "=" => {
                        let focal = info[3..4].parse::<u32>().unwrap();
                        let hashm = boxes.get_mut(&correct_box).unwrap();
                        if hashm.contains_key(&label){
                            let cur = hashm.get(&label).unwrap();
                            hashm.insert(label,(cur.0, focal));
                        }else{
                            hashm.insert(label,(order_index, focal));
                        }
                    },
                    "-" => {
                        boxes.get_mut(&correct_box).unwrap().remove(&label);
                    },
                    x => panic!("invalid operation: {}", x)
                }
                order_index += 1;
            });
    
    let mut res: u32 = 0;
    for i in 0..u8::MAX{
        let values = boxes.get(&i).unwrap().values();
        let ordering = values.sorted_by(|a, b| a.0.cmp(&b.0));
        for elem in ordering.enumerate(){
            println!("{} * {} * {}", i+1, elem.0 + 1, elem.1.1);
            res += (1+i as u32) * (elem.0 as u32 + 1) * elem.1.1;
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
