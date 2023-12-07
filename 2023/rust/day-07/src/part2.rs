use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

use crate::custom_error::AocError;

const CARD_STRENGTH: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
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

fn get_max_value_index(input: &HashMap<char, u8>) -> Option<char> {
    if let Some((&i, _)) = input
        .iter()
        .filter(|&(c, _)| *c != 'J')
        .max_by_key(|&(_, value)| value)
    {
        return Some(i);
    }
    None
}

fn calculate_hand_type(cards: &[char]) -> HandType {
    let mut repetition_index: HashMap<char, u8> = HashMap::new();
    cards.iter().for_each(|c| {
        match repetition_index.get(c) {
            Some(count) => repetition_index.insert(c.to_owned(), count + 1),
            None => repetition_index.insert(c.to_owned(), 1),
        };
    });

    let total = repetition_index.len();
    if total == 1 {
        return HandType::FiveOfKind;
    }

    let j_count = repetition_index.get(&'J').unwrap_or(&0).to_owned();

    if j_count != 0 {
        let max_char = get_max_value_index(&repetition_index).expect("Unable to get max char");
        let max_entry = repetition_index.get_mut(&max_char).unwrap();

        let max_entry = *max_entry + j_count;
        repetition_index.insert(max_char, max_entry);
        repetition_index.remove(&'J');
    }

    let total = repetition_index.len();
    if total == 1 {
        return HandType::FiveOfKind;
    }

    if total == 1 {
        return HandType::FiveOfKind;
    } else if total == 2 {
        let max_repetition = repetition_index.values().max().unwrap().to_owned();

        if max_repetition == 4 {
            return HandType::FourOfKind;
        } else {
            return HandType::FullHouse;
        }
    } else if total == 3 {
        let max_repetition = repetition_index.values().max().unwrap().to_owned();

        if max_repetition == 3 {
            return HandType::ThreeOfKind;
        } else {
            return HandType::TwoPair;
        }
    } else if total == 4 {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn get_card_strength(card: char) -> usize {
    CARD_STRENGTH.iter().position(|&c| c == card).unwrap()
}

fn sort_hands(hands: &[Hand]) -> Vec<&Hand> {
    let grouped_hands: Vec<Vec<&Hand>> = hands
        .iter()
        .sorted_by_key(|hand| &hand.hand_type)
        .group_by(|hand| &hand.hand_type)
        .into_iter()
        .fold(Vec::new(), |mut acc, (_, group)| {
            let grouped_items = group
                .into_iter()
                .sorted_by(|a, b| {
                    for i in 0..a.cards.len() {
                        let ac = get_card_strength(a.cards[i]);
                        let bc = get_card_strength(b.cards[i]);
                        match ac.cmp(&bc) {
                            Ordering::Equal => continue,
                            v => return v,
                        }
                    }

                    Ordering::Equal
                })
                .collect();
            acc.push(grouped_items);
            acc
        });

    grouped_hands.into_iter().flatten().collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let hands: Vec<Hand> = input.lines().map(Hand::parse).collect();
    let sorted_hands = sort_hands(&hands);

    let total = sorted_hands
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
