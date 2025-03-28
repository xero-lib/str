
use clap::{Parser, Subcommand};

pub enum Output {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Subcommand)]
pub enum Operation {
    /* Pattern-Based */
    #[command(
        about = "Splits at whitespace",
        long_about = "Splits each line at whitespace, optionally a finite number of times per line."
    )]
    SplitAtWhitespace {
        #[arg(
            help = "Optional: number of whitespace segments to split at (negative values start from end)"
        )]
        number: Option<i64>,
    },

    #[command(
        about = "Splits at a pattern",
        long_about = "Splits each line at a given pattern, optionally a finite number of times per line."
    )]
    SplitAtPat {
        #[arg(help = "Pattern at which to split the input")]
        pattern: String,
        #[arg(
            help = "Optional: number of times to split at given pattern (negative values start from end"
        )]
        number: Option<i64>,
    },

    #[command(
        about = "Splits at a character",
        long_about = "Splits each line at a given character, optionally a finite number of times per line."
    )]
    SplitAtChar {
        #[arg(help = "Character at which to split at.")]
        char: char,
        #[arg(
            help = "Optional: number of times to split at a given character (negative values start from end)"
        )]
        number: Option<i64>,
    },

    #[command(
        about = "Cuts starting at a pattern",
        long_about = "Cuts each line starting from a given pattern to the end of the line."
    )]
    CutFromPat {
        #[arg(
            help = "Pattern at which to begin cutting (pattern-inclusive, cuts until end of line)"
        )]
        pattern: String,
    },

    #[command(
        about = "Cuts between two patterns",
        long_about = "Cuts contents of a line between one pattern and another."
    )]
    CutFromPatToPat {
        #[arg(
            help = "Pattern at which to begin cutting (pattern-inclusive, cuts until end-pattern, or end of line)"
        )]
        start: String,
        #[arg(
            help = "Pattern at which to stop cutting (pattern-exclusive, from beginning if start not found)"
        )]
        end: String,
    },

    #[command(
        about = "Cuts from a pattern to an offset from the pattern",
        long_about = "Cuts starting from a pattern to a given offset amount from the first character of the pattern."
    )]
    CutFromPatToOffset {
        #[arg(help = "Pattern to begin cutting at (pattern-inclusive)")]
        pattern: String,
        #[arg(
            help = "Offset from pattern to cut until (negative values move backward from beginning of first pattern)"
        )]
        offset: i64,
    },

    #[command(
        about = "Cuts until a pattern",
        long_about = "Cuts starting from the beginning of a line to either the first character of a pattern, or the end of the line if the pattern isn't found."
    )]
    CutUntilPat {
        #[arg(help = "Pattern to cut until (pattern-exclusive)")]
        pattern: String,
    },

    #[command(
        about = "Trims a line starting from a pattern",
        long_about = "Trims (removes) contents of a line starting from a given pattern to the end of the line."
    )]
    TrimFromPat {
        #[arg(help = "Pattern to begin trimming at (pattern-inclusive)")]
        pattern: String,
    },

    #[command(
        about = "Trims between two patterns",
        long_about = "Trims (removes) contents of a line between one pattern and another, or the end of the line if the second pattern isn't found."
    )]
    TrimFromPatToPat {
        #[arg(
            help = "Pattern to begin trimming from (pattern-inclusive, trims to end if end-pattern is not found)"
        )]
        start: String,
        #[arg(help = "Pattern to stop trimming at (pattern-exclusive)")]
        end: String,
    },

    #[command(
        about = "Trims until a pattern",
        long_about = "Trims (removes) contents of a line starting from the beginning of the line, until the pattern is found. If the pattern isn't found, nothing is done."
    )]
    TrimUntilPat {
        #[arg(help = "Pattern to trim until (pattern-exclusive)")]
        pattern: String,
    },

    #[command(
        about = "Trims to a pattern (inclusive)",
        long_about = "Trims (removes) contents of a line up to and including a given pattern. If the pattern isn't found, nothing is done."
    )]
    TrimToPat {
        #[arg(help = "Pattern to trim to (pattern-inclusive)")]
        pattern: String,
    },

    /* Index-Based */
    #[command(
        about = "Splits at a given index",
        long_about = "Split each line at a given index."
    )]
    SplitAtIndex {
        #[arg(help = "Index to split at")]
        index: usize,
    },

    #[command(
        about = "Cut beginning at a given index",
        long_about = "Cut each line beginning at an index until the end of the line."
    )]
    CutFromIndex {
        #[arg(help = "Index to begin cutting from (index-inclusive, cuts until end of line)")]
        index: usize,
    },

    #[command(
        about = "Cut between two indices",
        long_about = "Cut each line between one index (inclusive) and another (exclusive)."
    )]
    CutFromIndexToIndex {
        #[arg(help = "Index to begin cutting from (index-inclusive)")]
        start: usize,
        #[arg(help = "Index to stop cutting at (index-exclusive)")]
        end: usize,
    },

    #[command(
        about = "Cuts from an index to an offset from the index",
        long_about = "Cuts contents of a line starting from an index to a given offset amount from the index."
    )]
    CutFromIndexToOffset {
        #[arg(help = "Index at which to begin cutting (index-inclusive)")]
        index: usize,
        #[arg(help = "Offset from index to cut to (negative values move backward from index)")]
        offset: i64,
    },

    #[command(
        about = "Cuts until a given index",
        long_about = "Cuts contents of a line starting from the beginning of each line to a given index in that line."
    )]
    CutUntilIndex {
        #[arg(help = "Index to cut until (index-exclusive, if zero)")]
        index: usize,
    },

    #[command(
        about = "Trims starting from an index",
        long_about = "Trims (removes) contents of a line starting from a given index (inclusive) the the end of the line."
    )]
    TrimFromIndex {
        #[arg(help = "Index to begin trimming from (zero-based, index-inclusive)")]
        index: usize,
    },

    #[command(
        about = "Trims between two indices",
        long_about = "Trims (removes) contents of a line between two indices."
    )]
    TrimFromIndexToIndex {
        #[arg(help = "Index to begin trimming from (zero-based, index-inclusive)")]
        start: usize,
        #[arg(help = "Index to stop trimming at (zero-based, index-exclusive)")]
        end: usize,
    },

    #[command(
        about = "Trims from an index ton an offset from the index",
        long_about = "Trims (removes) starting from an index to a given offset amount from the index."
    )]
    TrimFromIndexToOffset {
        #[arg(help = "Index to start trimming from (zero-based, index-inclusive)")]
        index: usize,
        #[arg(help = "Offset from index to trim to (negative values move backward from index)")]
        offset: i64,
    },

    #[command(
        about = "Trims until an index",
        long_about = "Trims (removes) contents of a line until a given index."
    )]
    TrimUntilIndex {
        #[arg(help = "Index to trim until (zero-baesd, index-exclusive)")]
        index: usize,
    },

    #[command(
        about = "Trims whitespace or patterns from both ends",
        long_about = "Trims (removes) either whitespace (default) or patterns from the beginning and end of each line."
    )]
    Trim {
        #[arg(help = "Optional: pattern to trim from beginning and end of input")]
        pattern: Option<String>,
    },

    #[command(
        about = "Replaces one pattern with another",
        long_about = "Replaces instances of a given pattern with another, optionally a finite number of times per line (to remove patterns entirely, use remove command)."
    )]
    Replace {
        #[arg(help = "Pattern to replace inline from input")]
        pattern: String,
        #[arg(help = "What to replace pattern with")]
        with: String,
        #[arg(
            help = "Optional: number of pattern-matches to replace (negative values start from end)"
        )]
        number: Option<i64>,
    },

    #[command(
        about = "Removes a pattern",
        long_about = "Removes inline a given pattern, optioanlly a finite number of times per line."
    )]
    Remove {
        #[arg(help = "Pattern to remove inline from input")]
        pattern: String,
        #[arg(
            help = "Optional: number of pattern-matches to remove (negative values start from end)"
        )]
        number: Option<i64>,
    },

    /* Mixed */
    #[command(
        about = "Cuts from a pattern to an index",
        long_about = "Cuts contents of a line starting at the first character of a pattern to a given index (exclusive)."
    )]
    CutFromPatToIndex {
        #[arg(help = "Pattern to begin cutting at (pattern-inclusive)")]
        pattern: String,
        #[arg(
            help = "Index to stop cutting at, must be after pattern (zero-based, index-exclusive)"
        )]
        index: usize,
    },

    #[command(
        about = "Cuts from an index to a pattern",
        long_about = "Cuts contents of a line starting from a given index (inclusive) and the first character of a pattern."
    )]
    CutFromIndexToPat {
        #[arg(help = "Index to begin cutting at (zero-based, index-inclusive)")]
        index: usize,
        #[arg(help = "Pattern to stop cutting at (pattern-exclusive)")]
        pattern: String,
    },

    #[command(
        about = "Trims between a pattern and an index",
        long_about = "Trims (removes) contents of a line between the firts character of a pattern and a given index in the line."
    )]
    TrimFromPatToIndex {
        #[arg(help = "Patter to begin trimming from (pattern-inclusive)")]
        pattern: String,
        #[arg(help = "Index to stop trimming at (index-exclusive)")]
        index: usize,
    },

    #[command(
        about = "Trims between an index and a pattern",
        long_about = "Trims (removes) contents of a line starting from a given index (inclusive) and the first character of a pattern."
    )]
    TrimFromIndexToPat {
        #[arg(help = "Index to begin trimming from (index-inclusive)")]
        index: usize,
        #[arg(help = "Pattern to stop trimming at (pattern-exclusive)")]
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
        use op_functions::*;
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
            TrimToPat { pattern } => trim_to_pat(pattern, input),
            Trim { pattern } => trim(pattern, input),
            Replace {
                pattern,
                with,
                number,
            } => replace(pattern, with, *number, input),
            Remove { pattern, number } => replace(pattern, &"".to_string(), *number, input),

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

mod op_functions {
    use super::Output;

    /* Pattern-Based */
    pub fn split_at_whitespace(number: Option<i64>, input: &String) -> Output {
        // define as closure to defer execution in case it's not needed
        let trimmed = || input.split_whitespace().map(str::to_owned);

        use Output::*;
        match number {
            None => Multiple(input.split_whitespace().map(str::to_owned).collect()),
            Some(x) if x.is_negative() => {
                Multiple(trimmed().rev().take(x.abs() as usize).collect())
            } // not exactly intended behavior, collect remaining and return as one entry
            Some(x) if x.is_positive() => Multiple(trimmed().take(x as usize).collect()),
            _ => Single(input.to_owned()),
        }
    }

    pub fn split_at_pat(number: Option<i64>, pattern: &String, input: &String) -> Output {
        use super::Output::*;
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

    pub fn split_at_char(number: Option<i64>, char: char, input: &String) -> Output {
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

    pub fn cut_from_pat(pattern: &String, input: &String) -> Output {
        Output::Single(input[input.find(pattern).unwrap_or(0)..].to_string())
    }

    pub fn cut_from_pat_to_pat(start: &String, end: &String, input: &String) -> Output {
        Output::Single(
            input[input.find(start).unwrap_or(0)..input.rfind(end).unwrap_or(input.capacity())]
                .to_string(),
        )
    }

    // separate fn for cut from last pat?
    pub fn cut_from_pat_to_offset(pattern: &String, offset: i64, input: &String) -> Output {
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
    pub fn cut_until_pat(pattern: &String, input: &String) -> Output {
        Output::Single(input[..input.find(pattern).unwrap_or(input.len())].to_string())
    }

    pub fn trim_from_pat(pattern: &String, input: &String) -> Output {
        Output::Single(input[input.find(pattern).unwrap_or(0)..].to_string())
    }

    pub fn trim_from_pat_to_pat(start: &String, end: &String, input: &String) -> Output {
        Output::Single(
            input[input.find(start).unwrap_or(0)..input.rfind(end).unwrap_or(input.len() - 1)]
                .to_string(),
        )
    }

    // separate fn for trim until last pat?
    pub fn trim_until_pat(pattern: &String, input: &String) -> Output {
        Output::Single(input[input.find(pattern).unwrap_or(0)..].to_string())
    }

    pub fn trim_to_pat(pattern: &String, input: &String) -> Output {
        Output::Single(input[input.find(pattern).unwrap_or(0) + input.len()..].to_string())
    }

    pub fn trim(pattern: &Option<String>, input: &String) -> Output {
        Output::Single(match pattern {
            None => input.trim().to_owned(),
            Some(p) => input
                .to_owned()
                .trim_start_matches(p)
                .trim_end_matches(p)
                .to_owned(),
        })
    }

    pub fn replace(pattern: &String, with: &String, number: Option<i64>, input: &String) -> Output {
        Output::Single(match number {
            None => input
                .split(pattern)
                .collect::<Vec<&str>>()
                .join(with.as_str()),
            Some(x) if x.is_negative() => input.rsplitn(x.abs() as usize, pattern).collect(),
            Some(0) => input.to_owned(),
            Some(x) => input.splitn(x as usize, pattern).collect(),
        })
    }

    // pub fn remove(pattern: &String, number: Option<i64>, input: &String) -> Output {
    //     Output::Single(
    //         match number {
    //             None => input.split(pattern).collect(),
    //             Some(x) if x.is_negative() => input.rsplitn(x.abs() as usize, pattern).collect(),
    //             Some(0) => input.to_owned(),
    //             Some(x) => input.splitn(x as usize, pattern).collect()
    //         }
    //     )
    // }

    /* Index-Based */
    pub fn cut_from_index(index: usize, input: &String) -> Output {
        Output::Single(input[index..].to_string())
    }

    pub fn cut_from_index_to_index(start: usize, end: usize, input: &String) -> Output {
        Output::Single(input[start..end].to_string())
    }

    pub fn cut_from_index_to_offset(index: usize, offset: i64, input: &String) -> Output {
        Output::Single(
            input[if offset.is_negative() {
                index + (offset as usize - 1)..index
            } else {
                index..index + offset as usize
            }]
            .to_string(),
        )
    }

    pub fn cut_until_index(index: usize, input: &String) -> Output {
        Output::Single(input[..if index != 0 { index } else { input.len() }].to_string())
    }

    pub fn trim_from_index(index: usize, input: &String) -> Output {
        Output::Single(input[..if index != 0 { index } else { input.len() }].to_string())
    }

    pub fn trim_from_index_to_index(start: usize, end: usize, input: &String) -> Output {
        Output::Single(if end <= start {
            input.clone()
        } else {
            input[..start].to_string() + &input[end..]
        })
    }

    pub fn trim_from_index_to_offset(index: usize, offset: i64, input: &String) -> Output {
        Output::Single(if offset.is_negative() {
            input[..index + offset as usize].to_string() + &input[index + 1..]
        } else {
            input[..index].to_string() + &input[index + offset as usize..]
        })
    }

    pub fn trim_until_index(index: usize, input: &String) -> Output {
        Output::Single(input[index..].to_string())
    }

    pub fn cut_from_pat_to_index(pattern: &String, index: usize, input: &String) -> Output {
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

    pub fn cut_from_index_to_pat(index: usize, pattern: &String, input: &String) -> Output {
        let found_idx = input.find(pattern).unwrap_or(0);
        if index > found_idx {
            eprintln!("First pattern instance was found after desired index");
            std::process::exit(1);
        } else if index == found_idx {
            return Output::Single(input.clone());
        }

        Output::Single(input[index..found_idx].to_string())
    }

    pub fn trim_from_pat_to_index(pattern: &String, index: usize, input: &String) -> Output {
        let found_idx = input.find(pattern).unwrap_or(0);

        if index < found_idx {
            eprintln!("Pattern was found before desired index");
            std::process::exit(1);
        } else if index == found_idx {
            return Output::Single(input.clone());
        }

        Output::Single(input[..found_idx].to_string() + &input[index..])
    }

    pub fn trim_from_index_to_pat(index: usize, pattern: &String, input: &String) -> Output {
        let found_idx = input.find(pattern).unwrap_or(0);

        if index > found_idx {
            eprintln!("First pattern was found after desired index");
            std::process::exit(1);
        } else if index == found_idx {
            return Output::Single(input.clone());
        }

        Output::Single(input[..index].to_string() + &input[found_idx..])
    }
}

#[derive(Parser)]
#[command(version, about = "Powerful command-line-driven string manipulation tool", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub operation: Option<Operation>,
}
