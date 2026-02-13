use beciarz_core::{greek_to_official, official_to_greek};
use clap::{Parser, ValueEnum};
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "beciarz", about = "tłumaczy alfabety")]
struct Cli {
    /// Input format (g/grecki or o/oficjalny)
    #[arg(
        short = 'i',
        long = "input-format",
        value_enum,
        default_value = "oficjalny"
    )]
    input_fmt: Format,

    /// Output format (g/grecki or o/oficjalny)
    #[arg(
        short = 'o',
        long = "output-format",
        value_enum,
        default_value = "grecki"
    )]
    output_fmt: Format,

    /// Do not print the trailing newline (like echo -n)
    #[arg(short = 'n')]
    no_newline: bool,

    /// The text to transform. If omitted, reads from stdin.
    input: Option<String>,
}
#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    /// Official / Oficjalny
    #[clap(name = "oficjalny", alias = "o")]
    Official,
    /// Greek / Grecki
    #[clap(name = "grecki", alias = "g")]
    Greek,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // 1. Resolve the text
    let text = match cli.input {
        Some(t) => t,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    // 2. Logic Matrix
    // We only have two functions, so we handle the 4 possible combinations:
    let result = match (cli.input_fmt, cli.output_fmt) {
        (Format::Official, Format::Greek) => official_to_greek(&text),
        (Format::Greek, Format::Official) => greek_to_official(&text),
        _ => {
            // If input and output are the same, just return the text as is
            text
        }
    };

    // 3. Output
    if cli.no_newline {
        print!("{result}");
    } else {
        println!("{result}");
    }

    Ok(())
}
