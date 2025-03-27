use clap::{Parser, Subcommand};

enum Output {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Subcommand)]
enum Operation {
    /* Pattern-Based */
    SplitAtWhitespace {
        #[arg(help = "Optional: number of whitespace segments to split at (negative values start from end)")]
        number: Option<i64>,
    },
    SplitAtPat {
        #[arg(help = "Pattern at which to split the input")]
        pattern: String,
        #[arg(help = "Optional: number of times to split at given pattern (negative values start from end")]
        number: Option<i64>,
    },
    SplitAtChar {
        #[arg(help = "Character at which to split at.")]
        char: char,
        #[arg(help = "Optional: number of times to split at given character (negative values start from end)")]
        number: Option<i64>,
    },
    CutFromPat {
        #[arg(help = "Pattern at which to begin cutting (pattern-inclusive, cuts until end of line)")]
        pattern: String,
    },
    CutFromPatToPat {
        #[arg(help = "Pattern at which to begin cutting (pattern-inclusive, cuts until end-pattern, or end of line)")]
        start: String,
        #[arg(help = "Pattern at which to stop cutting (pattern-exclusive, from beginning if start not found)")]
        end: String,
    },
    CutFromPatToOffset {
        #[arg(help = "Pattern to begin cutting at (pattern-inclusive)")]
        pattern: String,
        #[arg(help = "Offset from pattern to cut until (negative values move backward from beginning of first pattern)")]
        offset: i64,
    },
    CutUntilPat {
        #[arg(help = "Pattern to cut until (pattern-exclusive)")]
        pattern: String,
    },
    TrimFromPat {
        #[arg(help = "Pattern to begin trimming at (pattern-inclusive)")]
        pattern: String,
    },
    TrimFromPatToPat {
        #[arg(help = "Pattern to begin trimming from (pattern-inclusive, trims to end if end-pattern is not found)")]
        start: String,
        #[arg(help = "Pattern to stop trimming at (pattern-exclusive)")]
        end: String,
    },
    TrimUntilPat {
        #[arg(help = "Pattern to trim until (pattern-exclusive)")]
        pattern: String,
    },
    TrimToPat {
        #[arg(help = "Pattern to trim to (pattern-inclusive)")]
        pattern: String,
    },

    /* Index-Based */
    SplitAtIndex {
        #[arg(help = "Index to split at")]
        index: usize,
    },
    CutFromIndex {
        #[arg(help = "Index to begin cutting from (index-inclusive, cuts until end of line)")]
        index: usize,
    },
    CutFromIndexToIndex {
        #[arg(help = "Index to begin cutting from (index-inclusive)")]
        start: usize,
        #[arg(help = "Index to stop cutting at (index-exclusive)")]
        end: usize,
    },
    CutFromIndexToOffset {
        #[arg(help = "Index at which to begin cutting (index-inclusive)")]
        index: usize,
        #[arg(help = "Offset from index to cut to (negative values move backward from index)")]
        offset: i64,
    },
    CutUntilIndex {
        #[arg(help = "Index to cut until (index-exclusive, if zero)")]
        index: usize,
    },
    TrimFromIndex {
        #[arg(help = "Index to begin trimming from (zero-based, index-inclusive)")]
        index: usize,
    },
    TrimFromIndexToIndex {
        #[arg(help = "Index to begin trimming from (zero-based, index-inclusive)")]
        start: usize,
        #[arg(help = "Index to stop trimming at (zero-based, index-exclusive)")]
        end: usize,
    },
    TrimFromIndexToOffset {
        #[arg(help = "Index to start trimming from (zero-based, index-inclusive)")]
        index: usize,
        #[arg(help = "Offset from index to trim to (negative values move backward from index)")]
        offset: i64,
    },
    TrimUntilIndex {
        #[arg(help = "Index to trim until (zero-baesd, index-exclusive)")]
        index: usize,
    },
    Trim {
        #[arg(help = "Optional: pattern to trim from beginning and end of input")]
        pattern: Option<String>,
    },
    Replace {
        #[arg(help = "Pattern to replace inline from input")]
        pattern: String,
        #[arg(help = "What to replace pattern with (to remove patterns, use remove command)")]
        with: String,
        #[arg(help = "Optional: number of whitespace segments to split at (negative values start from end)")]
        number: Option<i64>,
    },

    Remove {
        #[arg(help = "Pattern to remove inline from input")]
        pattern: String,
        #[arg(help = "Optional: number of whitespace segments to split at (negative values start from end)")]
        number: Option<i64>,
    },

    /* Mixed */
    CutFromPatToIndex {
        #[arg(help = "Pattern to begin cutting at (pattern-inclusive)")]
        pattern: String,
        #[arg(help = "Index to stop cutting at, must be after pattern (zero-based, index-exclusive)")]
        index: usize,
    },
    CutFromIndexToPat {
        #[arg(help = "Index to begin cutting at (zero-based, index-inclusive)")]
        index: usize,
        #[arg(help = "Pattern to stop cutting at (pattern-exclusive)")]
        pattern: String,
    },
    TrimFromPatToIndex {
        #[arg(help = "Patter to begin trimming from (pattern-inclusive)")]
        pattern: String,
        #[arg(help = "Index to stop trimming at (index-exclusive)")]
        index: usize,
    },
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
        use Operation::*;
        use op_functions::*;

        match self {
            /* Pattern-Based */
            SplitAtWhitespace { number } => split_at_whitespace(*number, input),
            SplitAtPat { number, pattern } => split_at_pat(*number, pattern, input),
            SplitAtChar { number, char } => split_at_char(*number, *char, input),
            CutFromPat { pattern } => cut_from_pat(pattern, input),
            CutFromPatToPat { start, end } => cut_from_pat_to_pat(start, end, input),
            CutFromPatToOffset { pattern, offset } => cut_from_pat_to_offset(pattern, *offset, input),
            CutUntilPat { pattern } => cut_until_pat(pattern, input),
            TrimFromPat { pattern } => trim_from_pat(pattern, input),
            TrimFromPatToPat { start, end } => trim_from_pat_to_pat(start, end, input),
            TrimUntilPat { pattern } => trim_until_pat(pattern, input),
            TrimToPat { pattern } => trim_to_pat(pattern, input),
            Trim { pattern } => trim(pattern, input),
            Replace { pattern, with, number, } => replace(pattern, with, *number, input),
            Remove { pattern, number } => replace(pattern, &"".to_string(), *number, input),

            /* Index-Based */
            SplitAtIndex { index } => Output::Multiple({
                let (a, b) = input.split_at(*index);
                vec![a.into(), b.into()]
            }),
            CutFromIndex { index } => cut_from_index(*index, input),
            CutFromIndexToIndex { start, end } => cut_from_index_to_index(*start, *end, input),
            CutFromIndexToOffset { index, offset } => cut_from_index_to_offset(*index, *offset, input),
            CutUntilIndex { index } => cut_until_index(*index, input),
            TrimFromIndex { index } => trim_from_index(*index, input),
            TrimFromIndexToIndex { start, end } => trim_from_index_to_index(*start, *end, input),
            TrimFromIndexToOffset { index, offset } => trim_from_index_to_offset(*index, *offset, input),
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
    use crate::Output;
    /* Pattern-Based */
    pub fn split_at_whitespace(number: Option<i64>, input: &String) -> Output {
        // define as closure to defer execution in case it's not needed
        let trimmed = || input.split_whitespace().map(str::to_owned);

        use Output::*;
        match number {
            None => Multiple(input.split_whitespace().map(str::to_owned).collect()),
            Some(x) if x.is_negative() => Multiple(trimmed().rev().take(x.abs() as usize).collect()), // not exactly intended behavior, collect remaining and return as one entry
            Some(x) if x.is_positive() => Multiple(trimmed().take(x as usize).collect()),
            _ => Single(input.to_owned()),
        }
    }

    pub fn split_at_pat(number: Option<i64>, pattern: &String, input: &String) -> Output {
        use crate::Output::*;
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
            input[
                input.find(start).unwrap_or(0)..input.rfind(end).unwrap_or(input.capacity())
            ]
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
            input[
                input.find(start).unwrap_or(0)..input.rfind(end).unwrap_or(input.len() - 1)
            ]
            .to_string())
    }

    // separate fn for trim until last pat?
    pub fn trim_until_pat(pattern: &String, input: &String) -> Output {
        Output::Single(input[input.find(pattern).unwrap_or(0)..].to_string())
    }

    pub fn trim_to_pat(pattern: &String, input: &String) -> Output {
        Output::Single(input[input.find(pattern).unwrap_or(0)+ input.len()..].to_string())
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

// Powerful command-line-driven string manipulation tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    operation: Option<Operation>,
}

fn main() {
    let operation: Operation = Args::parse().operation.unwrap_or_default(); // call at top to enable flags without stdin
    for i in std::io::stdin().lines().flatten() {
        match operation.execute(&i) {
            Output::Multiple(x) => println!("{}", x.join("\n")),
            Output::Single(x) => println!("{x}"),
        }
    }
}
