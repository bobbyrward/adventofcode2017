use std::collections::HashSet;

use anyhow::Result;
use clap::Clap;

use crate::{input, Command};

#[derive(Debug, Clap)]
pub enum Args {
    Part1,
    Part2,
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

fn find_highest_bank(banks: &[i64]) -> (usize, i64) {
    let result = banks
        .iter()
        .enumerate()
        .skip(1)
        .fold((0, &banks[0]), |max, current| {
            if max.1 < current.1 {
                current
            } else {
                max
            }
        });

    (result.0, *result.1)
}

fn redistribute_bank(banks: &mut [i64], mut index: usize, mut value: i64) -> &mut [i64] {
    banks[index] = 0;

    while value > 0 {
        index += 1;

        if index >= banks.len() {
            index = 0;
        }

        banks[index] += 1;
        value -= 1;
    }

    banks
}

fn reallocate_banks(banks: &[i64]) -> (usize, Vec<i64>) {
    let mut cycles = 0;
    let mut previous_states: HashSet<Vec<i64>> = HashSet::new();
    let mut current_state = banks.to_vec();

    previous_states.insert(current_state.clone());

    loop {
        cycles += 1;

        let (index, value) = find_highest_bank(&current_state);
        redistribute_bank(&mut current_state, index, value);

        if previous_states.contains(&current_state) {
            break;
        }

        previous_states.insert(current_state.clone());
    }

    (cycles, current_state)
}

fn reallocate_banks_until_repeated(banks: &[i64]) -> usize {
    let mut cycles = 0;
    let mut current_state = banks.to_vec();

    loop {
        cycles += 1;

        let (index, value) = find_highest_bank(&current_state);
        redistribute_bank(&mut current_state, index, value);

        if current_state == banks {
            break;
        }
    }

    cycles
}

fn part_one() -> Result<String> {
    Ok(reallocate_banks(
        &input("day06")?
            .split('\t')
            .map(|w| w.trim_end().parse::<i64>().map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()?,
    )
    .0
    .to_string())
}

fn part_two() -> Result<String> {
    let (_, repeated_state) = reallocate_banks(
        &input("day06")?
            .split('\t')
            .map(|w| w.trim_end().parse::<i64>().map_err(anyhow::Error::from))
            .collect::<Result<Vec<_>>>()?,
    );

    Ok(reallocate_banks_until_repeated(&repeated_state).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_find_highest_bank() -> Result<()> {
        assert_eq!(find_highest_bank(&[0, 2, 7, 0]), (2, 7));
        assert_eq!(find_highest_bank(&[2, 4, 1, 2]), (1, 4));
        assert_eq!(find_highest_bank(&[3, 1, 2, 3]), (0, 3));
        assert_eq!(find_highest_bank(&[0, 2, 3, 4]), (3, 4));
        assert_eq!(find_highest_bank(&[1, 3, 4, 1]), (2, 4));
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_redistribute_bank() -> Result<()> {
        assert_eq!(redistribute_bank(&mut [0, 2, 7, 0], 2, 7), vec![2, 4, 1, 2]);
        assert_eq!(redistribute_bank(&mut [2, 4, 1, 2], 1, 4), vec![3, 1, 2, 3]);
        assert_eq!(redistribute_bank(&mut [3, 1, 2, 3], 0, 3), vec![0, 2, 3, 4]);
        assert_eq!(redistribute_bank(&mut [0, 2, 3, 4], 3, 4), vec![1, 3, 4, 1]);
        assert_eq!(redistribute_bank(&mut [1, 3, 4, 1], 2, 4), vec![2, 4, 1, 2]);
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_reallocate_banks() -> Result<()> {
        assert_eq!(reallocate_banks(&[0, 2, 7, 0]).0, 5);
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_reallocate_banks_until_state() -> Result<()> {
        assert_eq!(reallocate_banks_until_repeated(&[2, 4, 2, 1]), 4);
        Ok(())
    }
}
