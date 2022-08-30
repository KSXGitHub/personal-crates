mod utils;

use pipe_trait::*;
use std::{
    cmp::Ordering,
    iter::{once, IntoIterator, Iterator},
    str::FromStr,
};
use utils::split_once;

pub struct Sequence(Inner);

enum Inner {
    Once(u32),
    Ascending(u32, u32),
    Descending(u32, u32),
}

use Inner::*;

impl FromStr for Sequence {
    type Err = String;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = u32::from_str(text) {
            return value.pipe(Once).pipe(Sequence).pipe(Ok);
        }

        if let Some((begin, end)) = split_once(text, '-') {
            let begin = begin
                .parse::<u32>()
                .map_err(|error| format!("Cannot parse {:?} as number: {}", begin, error))?;
            let end = end
                .parse::<u32>()
                .map_err(|error| format!("Cannot parse {:?} as number: {}", end, error))?;
            return Ok(Sequence(match begin.cmp(&end) {
                Ordering::Equal => Once(begin),
                Ordering::Less => Ascending(begin, end),
                Ordering::Greater => Descending(begin, end),
            }));
        };

        Err(format!("Invalid Syntax: {}", text))
    }
}

impl IntoIterator for Sequence {
    type Item = u32;
    type IntoIter = Box<dyn Iterator<Item = u32>>;

    fn into_iter(self) -> Self::IntoIter {
        let Sequence(inner) = self;

        match inner {
            Once(value) => Box::new(once(value)),
            Ascending(begin, end) => Box::new(begin..end),
            Descending(begin, end) => Box::new((end + 1..begin + 1).rev()),
        }
    }
}

#[test]
fn test_once() {
    assert_eq!(
        "12".parse::<Sequence>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [12],
    );
}

#[test]
fn test_ascending() {
    assert_eq!(
        "3-15"
            .parse::<Sequence>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    );
}

#[test]
fn test_descending() {
    assert_eq!(
        "15-3"
            .parse::<Sequence>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4],
    );
}
