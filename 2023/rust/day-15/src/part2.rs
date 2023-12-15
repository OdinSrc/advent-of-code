use std::fmt::Display;

macro_rules! str_to_single_number {
    ($input: expr) => {
        $input.bytes().fold(0, |current_value, b| {
            let c = current_value + b as usize;
            (c + (c << 4)) & 0xFF
        })
    };
}

pub fn run(input: &str) -> impl Display {
    let input = input.trim();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    input
        .split(',')
        .for_each(|line| match line.split_once('=') {
            Some((label, focal)) => {
                let box_val = str_to_single_number!(label);

                let b = boxes
                    .get_mut(box_val)
                    .unwrap_or_else(|| panic!("Invalid box num: {box_val}"));
                let label_pos = b.iter().position(|(l1, _)| *l1 == label);

                if let Some(label_pos) = label_pos {
                    let label_element = b.get_mut(label_pos).unwrap();
                    *label_element = (label, focal.parse().unwrap());
                } else {
                    b.push((label, focal.parse().unwrap()));
                }
            }
            _ => {
                let label: &str = &line[0..line.len() - 1];

                let box_val = str_to_single_number!(label);

                let b = boxes.get_mut(box_val).unwrap();
                b.retain(|(l1, _)| *l1 != label);
            }
        });

    boxes
        .iter()
        .enumerate()
        .map(|(box_num, b)| {
            if b.is_empty() {
                return 0;
            }
            let box_num = box_num + 1;

            b.iter()
                .enumerate()
                .map(|(i, (_, focal))| (i + 1) * focal * box_num)
                .sum::<usize>()
        })
        .sum::<usize>()
}

use crate::custom_error::AocError;
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(run(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() -> miette::Result<()> {
        // let input = "HASH";
        // assert_eq!(52, str_to_single_number(input));

        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
