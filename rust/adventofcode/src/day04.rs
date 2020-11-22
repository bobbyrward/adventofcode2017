use std::collections::{HashMap, HashSet};

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

fn has_duplicate_words(line: &str) -> bool {
    let mut counter: HashSet<&str> = HashSet::new();

    let found = line.split(' ').find(|word| !counter.insert(word));

    found.is_some()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CharCount {
    what: char,
    count: usize,
}

impl CharCount {
    fn new(what: char, count: usize) -> Self {
        Self { what, count }
    }
}

fn has_anagrams(line: &str) -> bool {
    let mut counter: HashSet<Vec<CharCount>> = HashSet::new();

    let found = line
        .split(' ')
        .map(|word| {
            let mut char_counts: HashMap<char, usize> = HashMap::new();

            word.chars().for_each(|c| {
                char_counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
            });

            let mut counts = char_counts
                .into_iter()
                .map(|(c, n)| CharCount::new(c, n))
                .collect::<Vec<_>>();
            counts.sort_by_key(|cc| cc.what);
            counts
        })
        .find(|word| !counter.insert(word.clone()));

    found.is_some()
}

fn part_one() -> Result<String> {
    Ok(input("day04")?
        .lines()
        .filter(|s| !has_duplicate_words(s))
        .count()
        .to_string())
}

fn part_two() -> Result<String> {
    Ok(input("day04")?
        .lines()
        .filter(|s| !has_duplicate_words(s) && !has_anagrams(s))
        .count()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_has_duplicate_words() -> Result<()> {
        /*
        For example:

            aa bb cc dd ee is valid.
            aa bb cc dd aa is not valid - the word aa appears more than once.
            aa bb cc dd aaa is valid - aa and aaa count as different words.
        */

        assert!(!has_duplicate_words("aa bb cc dd ee"));
        assert!(has_duplicate_words("aa bb cc dd aa"));
        assert!(!has_duplicate_words("aa bb cc dd aaa"));

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_get_char_counts() -> Result<()> {
        /*
        For example:

            abcde fghij is a valid passphrase.
            abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the first word.
            a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming another word.
            iiii oiii ooii oooi oooo is valid.
            oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.
        */

        assert!(!has_anagrams("abcde fghij"));
        assert!(has_anagrams("abcde xyz ecdab"));
        assert!(!has_anagrams("a ab abc abd abf abj"));
        assert!(!has_anagrams("iiii oiii ooii oooi oooo"));
        assert!(has_anagrams("oiii ioii iioi iiio"));

        Ok(())
    }
    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        Ok(())
    }
}
