use std::io::Read;

use clap::{Parser, Subcommand};

enum Output {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Subcommand)]
enum Operation {
    /* Pattern-Based */
    SplitAtWhitespace {
        number: Option<i64>, // negative values mean from right
    },
    SplitAtPat {
        number: Option<i64>,
        pattern: String,
    },
    SplitAtChar {
        number: Option<i64>,
        char: char,
    },
    CutFromPat {
        pattern: String,
    },
    CutFromPatToPat {
        start: String,
        end: String,
    },
    CutFromPatToOffset {
        pattern: String,
        offset: i64,
    },
    CutUntilPat {
        pattern: String,
    },
    TrimFromPat {
        pattern: String,
    },
    TrimFromPatToPat {
        start: String,
        end: String,
    },
    TrimUntilPat {
        pattern: String,
    },

    /* Index-Based */
    SplitAtIndex {
        index: usize,
    },
    CutFromIndex {
        index: usize,
    },
    CutFromIndexToIndex {
        start: usize,
        end: usize,
    },
    CutFromIndexToOffset {
        index: usize,
        offset: i64,
    },
    CutUntilIndex {
        index: usize,
    },
    TrimFromIndex {
        index: usize,
    },
    TrimFromIndexToIndex {
        start: usize,
        end: usize,
    },
    TrimFromIndexToOffset {
        index: usize,
        offset: i64,
    },
    TrimUntilIndex {
        index: usize,
    },

    /* Mixed */
    CutFromPatToIndex {
        pattern: String,
        index: usize,
    },
    CutFromIndexToPat {
        index: usize,
        pattern: String,
    },
    TrimFromPatToIndex {
        pattern: String,
        index: usize,
    },
    TrimFromIndexToPat {
        index: usize,
        pattern: String,
    },
}

impl Default for Operation {
    fn default() -> Self {
        Self::SplitAtWhitespace { number: None }
    }
}

impl Operation {
    pub fn execute(&self, input: &String) -> Output {
        use Operation::*;
        match self {
            /* Pattern-Based */
            SplitAtWhitespace { number } => split_at_whitespace(*number, input),
            SplitAtPat { number, pattern } => split_at_pat(*number, pattern, input),
            SplitAtChar { number, char } => split_at_char(*number, *char, input),
            CutFromPat { pattern } => cut_from_pat(pattern, input),
            CutFromPatToPat { start, end } => cut_from_pat_to_pat(start, end, input),
            CutFromPatToOffset { pattern, offset } => {
                cut_from_pat_to_offset(pattern, *offset, input)
            }
            CutUntilPat { pattern } => cut_until_pat(pattern, input),
            TrimFromPat { pattern } => trim_from_pat(pattern, input),
            TrimFromPatToPat { start, end } => trim_from_pat_to_pat(start, end, input),
            TrimUntilPat { pattern } => trim_until_pat(pattern, input),

            /* Index-Based */
            SplitAtIndex { index } => Output::Multiple({
                let (a, b) = input.split_at(*index);
                vec![a.into(), b.into()]
            }),
            CutFromIndex { index } => cut_from_index(*index, input),
            CutFromIndexToIndex { start, end } => cut_from_index_to_index(*start, *end, input),
            CutFromIndexToOffset { index, offset } => {
                cut_from_index_to_offset(*index, *offset, input)
            }
            CutUntilIndex { index } => cut_until_index(*index, input),
            TrimFromIndex { index } => trim_from_index(*index, input),
            TrimFromIndexToIndex { start, end } => trim_from_index_to_index(*start, *end, input),
            TrimFromIndexToOffset { index, offset } => {
                trim_from_index_to_offset(*index, *offset, input)
            }
            TrimUntilIndex { index } => trim_until_index(*index, input),

            /* Mixed */
            CutFromPatToIndex { pattern, index } => cut_from_pat_to_index(pattern, *index, input),
            CutFromIndexToPat { index, pattern } => cut_from_index_to_pat(*index, pattern, input),
            TrimFromPatToIndex { pattern, index } => trim_from_pat_to_index(pattern, *index, input),
            TrimFromIndexToPat { index, pattern } => trim_from_index_to_pat(*index, pattern, input),
        }
    }
}

/* Pattern-Based */
fn split_at_whitespace(number: Option<i64>, input: &String) -> Output {
    let trimmed = input.split_whitespace().map(str::to_owned);

    use Output::*;
    match number {
        None => Multiple(input.split_whitespace().map(str::to_owned).collect()),
        Some(x) if x.is_negative() => Multiple(trimmed.rev().take(x.abs() as usize).collect()), // not exactly intended behavior, collect remaining and return as one entry
        Some(x) if x.is_positive() => Multiple(trimmed.take(x as usize).collect()),
        _ => Single(input.to_owned()),
    }
}

fn split_at_pat(number: Option<i64>, pattern: &String, input: &String) -> Output {
    use Output::*;
    match number {
        None => Multiple(input.split(pattern).map(str::to_owned).collect()),
        Some(x) if x.is_negative() => Multiple(
            input
                .rsplitn(x.abs() as usize, pattern)
                .map(str::to_owned)
                .collect(),
        ),
        Some(x) if x.is_positive() => Multiple(
            input
                .splitn(x as usize, pattern)
                .map(str::to_owned)
                .collect(),
        ),
        _ => Single(input.to_owned()),
    }
}

fn split_at_char(number: Option<i64>, char: char, input: &String) -> Output {
    use Output::*;
    match number {
        None => Multiple(input.split(char).map(str::to_owned).collect()),
        Some(x) if x.is_negative() => Multiple(
            input
                .rsplitn(x.abs() as usize, char)
                .map(str::to_owned)
                .collect(),
        ),
        Some(x) if x.is_positive() => {
            Multiple(input.splitn(x as usize, char).map(str::to_owned).collect())
        }
        _ => Single(input.to_owned()),
    }
}

fn cut_from_pat(pattern: &String, input: &String) -> Output {
    let start = input.find(pattern).unwrap_or(0);
    Output::Single(input[start..].to_string())
}

fn cut_from_pat_to_pat(start: &String, end: &String, input: &String) -> Output {
    let start_idx = input.find(start).unwrap_or(0);
    let end_idx = input.rfind(end).unwrap_or(input.capacity() - 1);

    Output::Single(input[start_idx..end_idx].to_string())
}

// separate fn for cut from last pat?
fn cut_from_pat_to_offset(pattern: &String, offset: i64, input: &String) -> Output {
    let start_idx = input.find(pattern).unwrap_or(0);
    if offset as usize + start_idx > input.len() - 1 {
        eprintln!("Offset exits bounds of input");
        std::process::exit(1); // this could be better with anyhow
    }

    Output::Single(
        input[if offset.is_negative() {
            start_idx + (offset.abs() as usize)..start_idx
        } else {
            start_idx..start_idx + (offset as usize)
        }]
        .to_string(),
    )
}

// separate fn for cut until last pat?
fn cut_until_pat(pattern: &String, input: &String) -> Output {
    let found_idx = input.find(pattern).unwrap_or(input.len());
    Output::Single(input[..found_idx].to_string())
}

fn trim_from_pat(pattern: &String, input: &String) -> Output {
    let found_idx = input.find(pattern).unwrap_or(0);
    Output::Single(input[found_idx..].to_string())
}

fn trim_from_pat_to_pat(start: &String, end: &String, input: &String) -> Output {
    let start_idx = input.find(start).unwrap_or(0);
    let end_idx = input.rfind(end).unwrap_or(input.len() - 1);

    Output::Single(input[start_idx..end_idx].to_string())
}

// separate fn for trim until last pat?
fn trim_until_pat(pattern: &String, input: &String) -> Output {
    let found_idx = input.find(pattern).unwrap_or(0);

    Output::Single(input[found_idx..].to_string())
}

/* Index-Based */
fn cut_from_index(index: usize, input: &String) -> Output {
    Output::Single(input[index..].to_string())
}

fn cut_from_index_to_index(start: usize, end: usize, input: &String) -> Output {
    Output::Single(input[start..end].to_string())
}

fn cut_from_index_to_offset(index: usize, offset: i64, input: &String) -> Output {
    Output::Single(
        input[if offset.is_negative() {
            index + (offset as usize - 1)..index
        } else {
            index..offset as usize
        }]
        .to_string(),
    )
}

fn cut_until_index(index: usize, input: &String) -> Output {
    let index = if index != 0 { index } else { input.len() };
    Output::Single(input[..index].to_string())
}

fn trim_from_index(index: usize, input: &String) -> Output {
    let index = if index != 0 { index } else { input.len() };
    Output::Single(input[..index].to_string())
}

fn trim_from_index_to_index(start: usize, end: usize, input: &String) -> Output {
    Output::Single(if end <= start {
        input.clone()
    } else {
        input[..start].to_string() + &input[end..]
    })
}

fn trim_from_index_to_offset(start: usize, offset: i64, input: &String) -> Output {
    Output::Single(
        input[if offset.is_negative() {
            start + ((offset - 1) as usize)..start
        } else {
            start..start + offset as usize
        }]
        .to_string(),
    )
}

fn trim_until_index(index: usize, input: &String) -> Output {
    Output::Single(input[index..].to_string())
}

fn cut_from_pat_to_index(pattern: &String, index: usize, input: &String) -> Output {
    // implement only matching after the index?
    let found_idx = input.find(pattern).unwrap_or(0);
    if index < found_idx {
        eprintln!("Pattern was found before desired index");
        std::process::exit(1);
    } else if index == found_idx {
        return Output::Single(input.clone());
    }

    Output::Single(input[found_idx..index].to_string())
}

fn cut_from_index_to_pat(index: usize, pattern: &String, input: &String) -> Output {
    let found_idx = input.find(pattern).unwrap_or(0);
    if index > found_idx {
        eprintln!("First pattern instance was found after desired index");
        std::process::exit(1);
    } else if index == found_idx {
        return Output::Single(input.clone());
    }

    Output::Single(input[index..found_idx].to_string())
}

fn trim_from_pat_to_index(pattern: &String, index: usize, input: &String) -> Output {
    let found_idx = input.find(pattern).unwrap_or(0);

    if index < found_idx {
        eprintln!("Pattern was found before desired index");
        std::process::exit(1);
    } else if index == found_idx {
        return Output::Single(input.clone());
    }

    Output::Single(input[..found_idx].to_string() + &input[index..])
}

fn trim_from_index_to_pat(index: usize, pattern: &String, input: &String) -> Output {
    let found_idx = input.find(pattern).unwrap_or(0);

    if index > found_idx {
        eprintln!("First pattern was found after desired index");
        std::process::exit(1);
    } else if index == found_idx {
        return Output::Single(input.clone());
    }

    Output::Single(input[..index].to_string() + &input[found_idx..])
}

// Powerful command-line-driven string manipulation tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    operation: Option<Operation>,
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    match Args::parse().operation.unwrap_or_default().execute(&input) {
        Output::Multiple(x) => println!("{}", x.join("\n")),
        Output::Single(x) => println!("{x}"),
    }
}
