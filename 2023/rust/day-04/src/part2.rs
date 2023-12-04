use crate::custom_error::AocError;

type WinningNumbers = Vec<u32>;
type YourNumbers = Vec<u32>;
type Card = (WinningNumbers, YourNumbers);

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let normalized_lines: Vec<Card> = input
        .lines()
        .map(|line| {
            let (_, line_data) = line.split_once(':').unwrap();
            let (w_str, y_str) = line_data.split_once('|').unwrap();

            let w_numbers: WinningNumbers = w_str
                .split(' ')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            let y_numbers: YourNumbers = y_str
                .split(' ')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            (w_numbers, y_numbers)
        })
        .collect();

    let total_cards = normalized_lines.len();

    let card_instances: usize = normalized_lines
        .iter()
        .enumerate()
        .map(|(i, c)| count_sub_cards(&normalized_lines, i, c))
        .sum();

    let total = card_instances + total_cards;
    Ok(total.to_string())
}

fn card_winning_count(card: &Card) -> usize {
    let (w_numbers, y_numbers) = card;
    y_numbers.iter().filter(|n| w_numbers.contains(n)).count()
}

fn count_sub_cards(all_cards: &Vec<Card>, index: usize, card: &Card) -> usize {
    let w_count = card_winning_count(card);
    if w_count == 0 {
        0
    } else {
        let next_cards: Vec<&Card> = all_cards.iter().skip(index + 1).take(w_count).collect();
        let total_count: usize = next_cards.len()
            + next_cards
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let next_index = index + 1 + i;
                    count_sub_cards(all_cards, next_index, c)
                })
                .sum::<usize>();
        total_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
