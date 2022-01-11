use std::{fs, vec};
use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t = String::from("nums.txt"))]
    filename: String
}
fn main() {
    // --snip--
    let args = Args::parse();


    let filename = args.filename;
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("loaded the data");
    let raw_instructions:Vec<&str> = contents.split("\n").collect();
    let mut instructions:Vec<(&str, i32)> = vec![];
    for raw_instruction in raw_instructions {
        instructions.push(clean_instruction(raw_instruction))
    }
    let mut depth:i32 = 0;
    let mut height:i32 = 0;
    for instruction in instructions {
        if instruction.0 == "forward" {
            height += instruction.1;
        }
        if instruction.0 == "up" {
            depth -= instruction.1;
        }
        if instruction.0 == "down" {
            depth += instruction.1;
        }
    }
    println!("Result of multiplying the depth and height is {}", depth * height)
}

fn clean_instruction(raw_instruction: &str) -> (&str, i32) {
    let cleaned_instruction:Vec<&str> = raw_instruction.trim().split(" ").collect();
    let direction = cleaned_instruction.get(0).unwrap().trim();
    let value = i32::from_str(cleaned_instruction.get(1).unwrap().trim()).unwrap();
    return (direction, value)
}
