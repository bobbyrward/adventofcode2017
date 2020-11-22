use std::collections::HashMap;

use anyhow::Result;
use clap::Clap;
use tracing::debug;

use crate::{Command, Point};

const INPUT: usize = 289326;

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

fn distance_from_port(pos: Point) -> i64 {
    pos.x.abs() + pos.y.abs()
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct SpiralIter {
    direction: Direction,
    current: Option<Point>,
}

impl SpiralIter {
    fn new() -> Self {
        Self {
            direction: Direction::Right,
            current: None,
        }
    }
}

impl Iterator for SpiralIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.current {
            let new_pos = match self.direction {
                Direction::Right => {
                    let new_pos = pos + Point::new(1, 0);

                    if new_pos.x > new_pos.y {
                        self.direction = Direction::Up;
                    }

                    new_pos
                }
                Direction::Up => {
                    let new_pos = pos + Point::new(0, -1);

                    if new_pos.y == -new_pos.x {
                        self.direction = Direction::Left;
                    }

                    new_pos
                }
                Direction::Left => {
                    let new_pos = pos + Point::new(-1, 0);

                    if new_pos.x == new_pos.y {
                        self.direction = Direction::Down;
                    }

                    new_pos
                }
                Direction::Down => {
                    let new_pos = pos + Point::new(0, 1);

                    if new_pos.y == -new_pos.x {
                        self.direction = Direction::Right
                    }

                    new_pos
                }
            };

            self.current = Some(new_pos);
        } else {
            self.current = Some(Point::new(0, 0));
        }

        self.current
    }
}

fn surrounding_squares(pos: Point) -> Vec<Point> {
    vec![
        pos + Point::new(-1, -1),
        pos + Point::new(0, -1),
        pos + Point::new(1, -1),
        pos + Point::new(1, 0),
        pos + Point::new(1, 1),
        pos + Point::new(0, 1),
        pos + Point::new(-1, 1),
        pos + Point::new(-1, 0),
    ]
}

fn part_one() -> Result<String> {
    Ok(distance_from_port(SpiralIter::new().nth(INPUT - 1).unwrap()).to_string())
}

fn surrounding_values() -> impl Iterator<Item = i64> {
    let initial_state: Option<HashMap<Point, i64>> = None;

    SpiralIter::new().scan(initial_state, |state, pos| {
        debug!(pos = ?pos);

        if let Some(cache) = state {
            let value = surrounding_squares(pos)
                .iter()
                .filter_map(|x| cache.get(x))
                .sum();

            debug!(cache = ?cache);

            cache.insert(pos, value);

            Some(value)
        } else {
            let mut cache = HashMap::with_capacity(1024);
            cache.insert(Default::default(), 1);
            state.replace(cache);
            Some(1)
        }
    })
}

fn part_two() -> Result<String> {
    Ok(format!(
        "{:?}",
        surrounding_values().find(|x| *x > INPUT as i64)
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        /*
        17  16  15  14  13
        18   5   4   3  12
        19   6   1   2  11
        20   7   8   9  10
        21  22  23---> ...

        For example:

            Data from square 1 is carried 0 steps, since it's at the access port.
            Data from square 12 is carried 3 steps, such as: down, left, left.
            Data from square 23 is carried only 2 steps: up twice.
            Data from square 1024 must be carried 31 steps.
        */
        assert_eq!(distance_from_port(Point::new(0, 0)), 0);
        assert_eq!(distance_from_port(Point::new(2, -1)), 3);
        assert_eq!(distance_from_port(Point::new(0, 2)), 2);

        assert_eq!(distance_from_port(SpiralIter::new().next().unwrap()), 0);

        assert_eq!(distance_from_port(SpiralIter::new().nth(11).unwrap()), 3);

        assert_eq!(distance_from_port(SpiralIter::new().nth(22).unwrap()), 2);

        assert_eq!(distance_from_port(SpiralIter::new().nth(1023).unwrap()), 31);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_spiral_iter() -> Result<()> {
        let expected = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(0, -1),
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
        ];

        let actual: Vec<_> = SpiralIter::new().take(expected.len()).collect();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        /*
        147  142  133  122   59
        304    5    4    2   57
        330   10    1    1   54
        351   11   23   25   26
        362  747  806--->   ...
        */
        let expected = vec![
            1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362,
            747, 806,
        ];

        assert_eq!(
            surrounding_values()
                .take(expected.len())
                .collect::<Vec<_>>(),
            expected
        );

        Ok(())
    }
}
