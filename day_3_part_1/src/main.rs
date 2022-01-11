use std::{fs};
use clap::Parser;
use std::str::FromStr;

const report_line_length: i32 = 12;


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
    let mut cleaned_instructions:Vec<[i32; report_line_length as usize]> = vec![];
    for raw_instruction in raw_instructions {
        cleaned_instructions.push(clean_instruction(raw_instruction));
    }
    let mut gamma:[i32; report_line_length as usize] = [0; report_line_length as usize];
    for index in 0..report_line_length {
        gamma[index as usize] = calculate_most_common_number(index, &cleaned_instructions);
    }
    let mut epsilon:[i32; report_line_length as usize] = [0; report_line_length as usize];
    for index in 0..report_line_length {
        if gamma[index as usize] == 1 {
            epsilon[index as usize] = 0;
        }
        if gamma[index as usize] == 0 {
            epsilon[index as usize] = 1;
        }
    }
    let gamma_dec = to_dec_number(gamma);
    let epsilon_dec = to_dec_number(epsilon);
    println!("the result is: {}, gamma is: {}, epsilon is: {}", gamma_dec * epsilon_dec, gamma_dec, epsilon_dec)
}

fn clean_instruction(raw_instruction: &str) -> [i32; report_line_length as usize] {
    let cleaned_instructions:Vec<char> = raw_instruction.trim().chars().collect();
    let mut cleaned_instruction:[i32; report_line_length as usize] = [0; report_line_length as usize];
    for index in 0..report_line_length {
        cleaned_instruction[index as usize] = i32::from_str(&cleaned_instructions.get(index as usize).unwrap().to_string()).unwrap();
    }

    return cleaned_instruction;
}

fn calculate_most_common_number(index: i32, nums:&Vec<[i32; report_line_length as usize]>) -> i32 {
    let mut zeros_counter = 0;
    let mut ones_counter = 0;
    for array in nums {
        if array[index as usize] == 0 {
            zeros_counter += 1;
        }
        if array[index as usize] == 1 {
            ones_counter += 1;
        }
    }
    if zeros_counter > ones_counter {
        return 0;
    }
    if ones_counter > zeros_counter {
        return 1;
    }
    panic!("Equal number of zeros and ones");
}

fn to_dec_number(num: [i32; report_line_length as usize]) -> i32 {
    let mut num_s:String = String::from("");
    for index in 0..report_line_length {
        println!("The before operation at index {} binary representation is: {}", index, num_s);
        num_s  = num_s + &num[index as usize].to_string();
        println!("The after operation at index {} binary representation is: {}", index, num_s);
    }
    let result = isize::from_str_radix(&num_s, 2).unwrap();
    println!("The bianry representation is: {}", num_s);
    return result as i32
}   