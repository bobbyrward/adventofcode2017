use anyhow::Result;
use clap::Clap;
use tracing::debug;

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

fn row_checksum(row: &str) -> Result<i64> {
    let (min, max) = row.split('\t').fold(
        (std::u8::MAX as i64, std::u8::MIN as i64),
        |(mut min, mut max), x| {
            let value = crate::digits_to_i64(x.as_bytes()).expect("Invalid input");

            if value < min {
                debug!(min, value, "value < min");
                min = value;
            }

            if value > max {
                debug!(max, value, "value > max");
                max = value;
            }

            (min, max)
        },
    );

    Ok(max - min)
}

fn row_div_checksum(row: &str) -> Result<i64> {
    let values = row
        .split('\t')
        .map(|s| crate::digits_to_i64(s.as_bytes()))
        .collect::<Result<Vec<_>>>()?;

    let found = values
        .iter()
        .filter_map(|l| {
            let found = values
                .iter()
                .filter_map(|r| {
                    if *l == *r {
                        return None;
                    }

                    let max = std::cmp::max(*l, *r);
                    let min = std::cmp::min(*l, *r);

                    if max % min == 0 {
                        Some(*r)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !found.is_empty() {
                debug!(l = *l, r = found[0], "Found");
                Some((*l, found[0]))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let (l, r) = found[0];

    Ok(std::cmp::max(l, r) / std::cmp::min(l, r))
}

fn grid_checksum<F>(grid: &str, func: F) -> Result<i64>
where
    F: Fn(&str) -> Result<i64>,
{
    let rows = grid
        .lines()
        .map(|row| func(row.trim_end()))
        .collect::<Result<Vec<_>>>()?;

    Ok(rows.iter().sum())
}

fn part_one() -> Result<String> {
    grid_checksum(&input("day02")?, row_checksum).map(|i| i.to_string())
}

fn part_two() -> Result<String> {
    grid_checksum(&input("day02")?, row_div_checksum).map(|i| i.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        /*
        For example, given the following spreadsheet:

        5 1 9 5
        7 5 3
        2 4 6 8

        The first row's largest and smallest values are 9 and 1, and their difference is 8.
        The second row's largest and smallest values are 7 and 3, and their difference is 4.
        The third row's difference is 6.


        In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
        */
        assert_eq!(row_checksum("5\t1\t9\t5")?, 8);
        assert_eq!(row_checksum("7\t5\t3")?, 4);
        assert_eq!(row_checksum("2\t4\t6\t8")?, 6);

        assert_eq!(
            grid_checksum("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8\n", row_checksum)?,
            18
        );

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        /*
        For example, given the following spreadsheet:

        5 9 2 8
        9 4 7 3
        3 8 6 5

            In the first row, the only two numbers that evenly divide are 8 and 2; the result of this division is 4.
            In the second row, the two numbers are 9 and 3; the result is 3.
            In the third row, the result is 2.

        In this example, the sum of the results would be 4 + 3 + 2 = 9.
        */
        assert_eq!(row_div_checksum("5\t9\t2\t8")?, 4);
        assert_eq!(row_div_checksum("9\t4\t7\t3")?, 3);
        assert_eq!(row_div_checksum("3\t8\t6\t5")?, 2);

        assert_eq!(
            grid_checksum("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5\n", row_div_checksum)?,
            9
        );
        Ok(())
    }
}
