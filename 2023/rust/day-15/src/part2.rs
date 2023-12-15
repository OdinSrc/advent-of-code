use std::fmt::Display;

macro_rules! hash {
    ($input: expr) => {
        $input.bytes().fold(0, |current_value: u8, b| {
            current_value.wrapping_add(b).wrapping_mul(17)

            // (c.wrapping_add(c << 4))
        }) & 0xFF
    };
}

struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

// Code cleanup based on https://nickymeuleman.netlify.app/garden/aoc2023-day15
enum Instruction<'a> {
    Add(Lens<'a>),
    Remove(&'a str),
}

impl<'a> Instruction<'a> {
    fn new(input: &'a str) -> Self {
        if let Some(label) = input.strip_suffix('-') {
            Self::Remove(label)
        } else {
            let (label, focal) = input.split_once('=').unwrap();
            let focal = focal.parse().unwrap();
            let lens = Lens { label, focal };
            Self::Add(lens)
        }
    }
}

pub fn run(input: &str) -> impl Display {
    let input = input.trim();
    const BOX: Vec<Lens> = Vec::new();
    let mut boxes = [BOX; 256];

    input
        .split(',')
        .map(Instruction::new)
        .for_each(|instr| match instr {
            Instruction::Add(lens) => {
                let hash = hash!(lens.label);

                let lenses = &mut boxes[hash as usize];

                if let Some(old) = lenses.iter_mut().find(|item| lens.label == item.label) {
                    *old = lens;
                } else {
                    lenses.push(lens);
                }
            }
            Instruction::Remove(label) => {
                let hash = hash!(label);
                boxes[hash as usize].retain(|item| item.label != label);
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
                .map(|(i, lens)| (i + 1) * lens.focal as usize * box_num)
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
