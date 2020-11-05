use pipe_trait::*;
use std::{
    cmp::Ordering,
    iter::{once, IntoIterator, Iterator},
    path::PathBuf,
    process::exit,
    str::FromStr,
};
use structopt::StructOpt;
use structopt_utilities::StructOptUtils;

#[derive(StructOpt)]
struct CliArgs {
    /// Sequences of display id ranges, e.g. 0-3
    #[structopt(default_value = "255-0")]
    sequences: Vec<Sequence>,
}

struct Sequence(Box<dyn Iterator<Item = u32>>);

impl FromStr for Sequence {
    type Err = String;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = u32::from_str(text) {
            return Ok(Sequence(Box::new(once(value))));
        }

        if let Some((begin, end)) = split_once(text, '-') {
            let begin = begin
                .parse::<u32>()
                .map_err(|error| format!("Cannot parse {:?} as number: {}", begin, error))?;
            let end = end
                .parse::<u32>()
                .map_err(|error| format!("Cannot parse {:?} as number: {}", end, error))?;
            return Ok(Sequence(match begin.cmp(&end) {
                Ordering::Equal => Box::new(once(begin)),
                Ordering::Less => Box::new(begin..end),
                Ordering::Greater => Box::new((end + 1..begin + 1).rev()),
            }));
        };

        Err(format!("Invalid Syntax: {}", text))
    }
}

impl IntoIterator for Sequence {
    type Item = u32;
    type IntoIter = Box<dyn Iterator<Item = u32>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}

fn split_once(text: &str, sep: char) -> Option<(&str, &str)> {
    for (current_index, current_char) in text.char_indices() {
        if current_char == sep {
            return Some((&text[..current_index], &text[current_index + 1..]));
        }
    }

    None
}

fn main() {
    let CliArgs { sequences } = CliArgs::strict_from_args();

    let display = sequences
        .into_iter()
        .flatten()
        .map(|x| x.to_string())
        .find(|x| !format!("/tmp/.X{}-lock", x).pipe(PathBuf::from).exists());

    if let Some(display) = display {
        println!("{}", display);
    } else {
        exit(1);
    }
}

#[test]
fn test_sequence_once() {
    assert_eq!(
        "12".parse::<Sequence>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [12],
    );
}

#[test]
fn test_sequence_ascending() {
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
fn test_sequence_descending() {
    assert_eq!(
        "15-3"
            .parse::<Sequence>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4],
    );
}

#[test]
fn test_split_once_positive() {
    assert_eq!(split_once("0-255", '-'), Some(("0", "255")));
}

#[test]
fn test_split_once_negative() {
    assert_eq!(split_once("0-255", ':'), None);
}
