use std::fs;
use std::str::FromStr;
use clap::Parser;

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
    let nums:Vec<&str> = contents.split("\n").collect();
    let mut increases:i32 = 0; 
    for index in 1..nums.len() {
        let num_1 = i32::from_str(&nums[index - 1][0..nums[index -1].len() - 1]).unwrap();
        let num_2 = i32::from_str(&nums[index][0..nums[index].len() - 1]).unwrap();
        if num_2 > num_1 {
            increases += 1;
            println!("found increase on index {}", index - 1);
        }
    }
    println!("Number of increases: {}", increases)
}
