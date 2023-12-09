use itertools::Itertools;
advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {

    fn from_char(input: char, second: bool) -> Card {
        if !second {
            match input {
                'A'  => Card::Ace,
                'K'  => Card::King,
                'Q'  => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _      => panic!("unmatched card"),
            }
        }else{
            match input {
                'A'  => Card::Ace,
                'K'  => Card::King,
                'Q'  => Card::Queen,
                'J' => Card::Joker,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _      => panic!("unmatched card"),
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}


#[derive(Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    key: String,
}

impl Hand {
    pub fn new(input: &str, second: bool) -> Self {
        let mut chars = input.chars();

        let cards = [
            Card::from_char(chars.next().unwrap(), second),
            Card::from_char(chars.next().unwrap(), second),
            Card::from_char(chars.next().unwrap(), second),
            Card::from_char(chars.next().unwrap(), second),
            Card::from_char(chars.next().unwrap(), second),
        ];
        let hand_type = Self::determine_type(&cards, second);

        Hand {
            cards,
            hand_type,
            key: input.to_owned()
        }
    }

    fn determine_type(hand: &[Card; 5], second: bool) -> HandType {
        let mut card_counts = hand.iter().counts_by(|h| h);

        let num_unique_card_type = card_counts.len();

        if second && card_counts.contains_key(&Card::Joker) {
            let joker_count = card_counts.remove(&Card::Joker).unwrap();

            match num_unique_card_type {
                1 => HandType::FiveOfAKind,
                2 => HandType::FiveOfAKind,
                5 => HandType::OnePair,
                3 => {
                    return match *card_counts.values().max().unwrap() + joker_count{
                        3 => HandType::FullHouse,
                        4 => HandType::FourOfAKind,
                        _ => panic!("no hand type")
                    };
                },
                4 => { //Should never be two Pairs but this makes it clearer to read the thinking process
                    return match *card_counts.values().max().unwrap() + joker_count{
                        2 => HandType::TwoPair,
                        3 => HandType::ThreeOfAKind,
                        _ => panic!("no hand type")
                    };
                }
                _ => panic!("no hand type")
            }  

        }else{
            match num_unique_card_type {
                1 => HandType::FiveOfAKind,
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                2 => {
                    return match *card_counts.values().next().unwrap(){
                        1 => HandType::FourOfAKind,
                        2 => HandType::FullHouse,
                        3 => HandType::FullHouse,
                        4 => HandType::FourOfAKind,
                        _ => panic!("no hand type")
                    };
                },
                3 => {
                    return match *card_counts.values().filter(|val| val > &&1).next().unwrap(){
                        2 => HandType::TwoPair,
                        3 => HandType::ThreeOfAKind,
                        _ => panic!("no hand type")
                    };
                }
                _ => panic!("no hand type")
            }
        }

        
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then(self.cards[0].cmp(&other.cards[0]))
            .then(self.cards[1].cmp(&other.cards[1]))
            .then(self.cards[2].cmp(&other.cards[2]))
            .then(self.cards[3].cmp(&other.cards[3]))
            .then(self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand_type.partial_cmp(&other.hand_type)
    }
}

impl PartialEq for Hand{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands =  input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            (Hand::new(parts.next().unwrap(), false), parts.next().unwrap().parse::<u32>().unwrap())
        })
        .collect::<Vec<(Hand, u32)>>();
    
    hands.sort_by(|a,b| b.cmp(a));

    // println!("{:#?}", hands);

    Some(
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, elem)| acc + (1 + index as u32) * elem.1)
    )
    
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands =  input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            (Hand::new(parts.next().unwrap(), true), parts.next().unwrap().parse::<u32>().unwrap())
        })
        .collect::<Vec<(Hand, u32)>>();
    
    hands.sort_by(|a,b| b.cmp(a));

    // println!("{:#?}", hands);

    Some(
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, elem)| acc + (1 + index as u32) * elem.1)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
