use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

use crate::custom_error::AocError;

const CARD_STRENGTH: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

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

fn calculate_hand_type(cards: &[char]) -> HandType {
    let mut repetition_index: HashMap<char, u8> = HashMap::new();
    cards.iter().for_each(|c| {
        match repetition_index.get(c) {
            Some(count) => repetition_index.insert(*c, count + 1),
            None => repetition_index.insert(*c, 1),
        };
    });

    let total = repetition_index.len();
    match total {
        1 => HandType::FiveOfKind,
        2 => {
            let max_repetition = *repetition_index.values().max().unwrap();
            if max_repetition == 4 {
                HandType::FourOfKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            let max_repetition = *repetition_index.values().max().unwrap();
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
    // let sorted_hands = sort_hands(hands);

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

    // dbg!(&sorted_hands);
    let total = hands
        .iter()
        .enumerate()
        // .for_each(|(i, hand)| {
        //     println!("{i} {} {}", hand.bid, hand.bid * (i as u32 + 1));
        // });
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
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
