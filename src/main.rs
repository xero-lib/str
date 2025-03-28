mod cli;

use cli::{ Operation, Args, Output };
use clap::Parser;

fn main() {
    let operation: Operation = Args::parse().operation.unwrap_or_default(); // call at top to enable flags without stdin
    for i in std::io::stdin().lines().flatten() {
        match operation.execute(&i) {
            Output::Multiple(x) => println!("{}", x.join("\n")),
            Output::Single(x) => println!("{x}"),
        }
    }
}
