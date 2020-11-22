use anyhow::Result;
use clap::Clap;
use tracing::debug;

use crate::{digit_to_u8, input, Command};

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

fn digitization_quarantine(input: &str) -> i64 {
    let bytes = input.as_bytes();

    let mut initial_sum: i64 = bytes
        .iter()
        .zip(bytes.iter().skip(1))
        .filter_map(|(l, r)| {
            if *l == *r {
                debug!(
                    l = ?std::char::from_u32(*l as u32),
                    r = ?std::char::from_u32(*r as u32),
                    "Match"
                );
                Some(digit_to_u8(*l).expect("Bad input data") as i64)
            } else {
                None
            }
        })
        .sum();

    if bytes.first().unwrap() == bytes.last().unwrap() {
        debug!(l = ?bytes.first(), r = ?bytes.last(), "Last Match");
        initial_sum += digit_to_u8(bytes[0]).expect("Bad input data") as i64;
    }

    initial_sum
}

fn digitization_quarantine_circular(input: &str) -> i64 {
    let bytes = input.as_bytes();

    let initial_sum: i64 = bytes
        .iter()
        .zip(bytes.iter().skip(bytes.len() / 2).chain(bytes.iter()))
        .filter_map(|(l, r)| {
            if *l == *r {
                debug!(
                    l = ?std::char::from_u32(*l as u32),
                    r = ?std::char::from_u32(*r as u32),
                    "Match"
                );
                Some(digit_to_u8(*l).expect("Bad input data") as i64)
            } else {
                None
            }
        })
        .sum();

    initial_sum
}

fn part_one() -> Result<String> {
    Ok(digitization_quarantine(&input("day01")?.trim()).to_string())
}

fn part_two() -> Result<String> {
    Ok(digitization_quarantine_circular(&input("day01")?.trim()).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        /*
        1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
        1111 produces 4 because each digit (all 1) matches the next.
        1234 produces 0 because no digit matches the next.
        91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
        */
        assert_eq!(digitization_quarantine("1122"), 3);
        assert_eq!(digitization_quarantine("1111"), 4);
        assert_eq!(digitization_quarantine("1234"), 0);
        assert_eq!(digitization_quarantine("91212129"), 9);
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        /*
        1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
        1221 produces 0, because every comparison is between a 1 and a 2.
        123425 produces 4, because both 2s match each other, but no other digit has a match.
        123123 produces 12.
        12131415 produces 4.
        */
        assert_eq!(digitization_quarantine_circular("1212"), 6);
        assert_eq!(digitization_quarantine_circular("1221"), 0);
        assert_eq!(digitization_quarantine_circular("123425"), 4);
        assert_eq!(digitization_quarantine_circular("123123"), 12);
        assert_eq!(digitization_quarantine_circular("12131415"), 4);

        Ok(())
    }
}
