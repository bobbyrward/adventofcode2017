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

fn increment_offset(offset: i64) -> i64 {
    offset + 1
}

fn increment_offset2(offset: i64) -> i64 {
    if offset >= 3 {
        offset - 1
    } else {
        offset + 1
    }
}

fn compile_steps(steps: &str) -> Result<Vec<i64>> {
    steps
        .lines()
        .map(|jump| jump.parse::<i64>().map_err(anyhow::Error::from))
        .collect()
}

fn steps_to_exit<F>(steps: &mut Vec<i64>, alter_offset_fn: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut idx = 0;
    let mut nsteps = 0;

    loop {
        nsteps += 1;
        let jmp: i64 = steps[idx];
        let absolute: i64 = idx as i64 + jmp;

        steps[idx] = alter_offset_fn(steps[idx]);

        if absolute < 0 || absolute >= steps.len() as i64 {
            return nsteps;
        }

        idx = absolute as usize;
    }
}

fn part_one() -> Result<String> {
    let mut steps = compile_steps(&input("day05")?)?;
    Ok(steps_to_exit(&mut steps, increment_offset).to_string())
}

fn part_two() -> Result<String> {
    let mut steps = compile_steps(&input("day05")?)?;
    Ok(steps_to_exit(&mut steps, increment_offset2).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let s = "0\n3\n0\n1\n-3";

        let mut steps = compile_steps(s)?;
        assert_eq!(steps, vec![0, 3, 0, 1, -3]);

        assert_eq!(steps_to_exit(&mut steps, increment_offset), 5);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let s = "0\n3\n0\n1\n-3";

        let mut steps = compile_steps(s)?;
        assert_eq!(steps, vec![0, 3, 0, 1, -3]);

        assert_eq!(steps_to_exit(&mut steps, increment_offset2), 10);
        assert_eq!(steps, vec![2, 3, 2, 3, -1]);

        Ok(())
    }
}
