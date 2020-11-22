#[macro_use]
mod args;
mod command;
mod point;

use anyhow::{anyhow, Context, Result};
use clap::Clap;
use tracing_subscriber::FmtSubscriber;

use crate::command::Command;
use crate::point::Point;

// NOTE: Each solution module must be added here
solution!(day01, day02, day03, day04, day05);

fn input(name: &str) -> Result<String> {
    Ok(
        std::fs::read_to_string(format!("inputs/{}.txt", name))
            .with_context(|| name.to_string())?,
    )
}

fn digit_to_u8(digit: u8) -> Result<u8> {
    if digit < b'0' || digit > b'9' {
        return Err(anyhow!("Non numeric digit: {}", digit));
    }

    Ok(digit - b'0')
}

#[allow(dead_code)]
/// Convert a slice of numeric bytes to an integer
fn digits_to_i64(mut digits: &[u8]) -> Result<i64> {
    let mut sign = 1;

    if !digits.is_empty() && digits[0] == b'-' {
        sign = -1;
        digits = &digits[1..];
    }

    if digits.iter().any(|byte| *byte < b'0' || *byte > b'9') {
        return Err(anyhow!("Non numeric digit in string: '{:?}'", digits));
    }

    Ok(digits
        .iter()
        .rev()
        .enumerate()
        .map(|(exp, byte)| {
            let digit: i64 = (byte - b'0').into();
            let tens = 10i64.pow(exp as u32);

            digit * tens
        })
        .sum::<i64>()
        * sign)
}

fn main() -> Result<()> {
    let args = args::Args::parse();

    FmtSubscriber::builder()
        .with_env_filter(args.env_filter())
        .init();

    let solution = args.command.execute()?;

    println!("Solution:\n{}", solution);

    Ok(())
}
