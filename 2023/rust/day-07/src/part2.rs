use std::cmp::Ordering;

use crate::custom_error::AocError;

const CARD_STRENGTH: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
const CHAR_INDEX: [u8; 128] = [0; 128];

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn parse(input: &str) -> Self {
        let (cards_part, bid_part) = input.split_once(' ').unwrap();
        let cards: Vec<char> = cards_part.chars().collect();

        Self {
            hand_type: calculate_hand_type(&cards),
            cards,
            bid: bid_part.parse().unwrap(),
        }
    }
}

fn get_max_value_index(input: &[u8]) -> Option<usize> {
    input
        .iter()
        .enumerate()
        .filter(|&(c, _)| c != 'J' as usize)
        .max_by_key(|&(_, value)| value)
        .map(|(i, _)| i)
}

fn calculate_hand_type(cards: &[char]) -> HandType {
    let mut repetition_index = CHAR_INDEX;

    for &c in cards {
        repetition_index[c as usize] += 1;
    }

    let total = repetition_index.iter().filter(|&&count| count > 0).count();

    if total == 1 {
        return HandType::FiveOfKind;
    }

    let j_count = repetition_index['J' as usize];

    if j_count != 0 {
        let max_char = get_max_value_index(&repetition_index).expect("Unable to get max char");
        let max_entry = &mut repetition_index[max_char as usize];

        *max_entry += j_count;
        repetition_index['J' as usize] = 0;
    }

    let total = repetition_index.iter().filter(|&&count| count > 0).count();
    if total == 1 {
        return HandType::FiveOfKind;
    }

    match total {
        1 => HandType::FiveOfKind,
        2 => {
            let max_repetition = *repetition_index.iter().max().unwrap();
            if max_repetition == 4 {
                HandType::FourOfKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            let max_repetition = *repetition_index.iter().max().unwrap();
            if max_repetition == 3 {
                HandType::ThreeOfKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut hands: Vec<Hand> = input.lines().map(Hand::parse).collect();

    hands.sort_by(|a, b| {
        let type_ordering = a.hand_type.cmp(&b.hand_type);
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for i in 0..a.cards.len() {
            let ac = CARD_STRENGTH.iter().position(|&c| c == a.cards[i]).unwrap();
            let bc = CARD_STRENGTH.iter().position(|&c| c == b.cards[i]).unwrap();

            let order = ac.cmp(&bc);
            if let Ordering::Equal = order {
                continue;
            }
            return order;
        }

        Ordering::Equal
    });

    let total = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>();
    // let total = "";
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}
