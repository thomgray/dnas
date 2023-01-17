mod domain;
mod parser;

use atty::Stream;
use clap;
use clap::Parser;
use colored::Colorize;
use std::io::{Result, Read};
use std::io;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    seed: Option<i32>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pretty: u8,

    #[arg(long)]
    no_color: bool,

    file_path: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let base_vec: Vec<domain::Base> = match args.seed {
        Some(f) => seeded(f),
        None => from_input(&args),
    }.expect("Something not right");

    if args.no_color {
        colored::control::set_override(false);
    }
    print_output(&base_vec, args.pretty);
    Ok(())
}

fn seeded(f: i32) -> Result<Vec<domain::Base>> {
    let mut i = 0;
    let mut vec = Vec::new();
    loop {
        if i >= f {
            break;
        };
        vec.push(domain::random_base());
        i += 1;
    }
    Result::Ok(vec)
}

fn from_input(args: &Cli) -> Result<Vec<domain::Base>> {
    let contents = match args.file_path.as_ref() {
        Some(fp) => {
            std::fs::read_to_string(fp)
                .expect("Invalid file path provided")
        },
        None => from_pipe_maybe().unwrap(),
    };
    let mut remainder: &str = contents.as_str();
    let mut base_vector: Vec<domain::Base> = Vec::new();
    loop {
        match parser::parse_base(remainder) {
            Ok((r, c)) => {
                remainder = r;
                match domain::Base::from_char(c) {
                    Ok(base) => base_vector.push(base),
                    Err(_) => (),
                }
            },
            Err(_) => break,
        }
    }
    Result::Ok(base_vector)
}

fn from_pipe_maybe() -> Result<String> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    return Ok(buffer);
}

fn print_output(base_vec: &Vec<domain::Base>, pretty: u8) {
    match pretty {
        0 => {
            base_vec.iter().for_each(|v| {
                print!("{}", coloured_base_str(v));
            });
        },
        1 => {
            let helix = "---".yellow();
            base_vec.iter().for_each(|v| {
                print!("{} {} {}\n", coloured_base_str(v), helix, coloured_base_str(&v.complement()));
            });
        },
        _ => {
            let spires = [1,3,7,9,9,9,7,3];
            let max_spire_w = 9;
            let modulo = spires.len();
            
            let mut i = 0;
            // let twist_length = 8;
            let dash = "-";
            base_vec.iter().for_each(|v| {
                let l = spires[i%modulo];
                // let mut modulo = i % twist_length;
                // if (modulo >= twist_length/2) {
                //     modulo = twist_length - modulo - 1;
                // }
                let helix = dash.repeat(l);
                // let ident_l = (twist_length/2)-modulo;
                let ident = " ".repeat((max_spire_w - l)/2);
                print!("{}{} {} {}\n", ident, coloured_base_str(v), helix.yellow(), coloured_base_str(&v.complement()));
                i += 1;
            });

        }    
    }
}

fn coloured_base_str(b: &domain::Base) -> colored::ColoredString {
    let col = match b {
        domain::Base::A => "red",
        domain::Base::C => "green",
        domain::Base::G => "magenta",
        domain::Base::T => "blue",
    };
    b.to_char().to_string().color(col)
}