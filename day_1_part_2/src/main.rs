use std::fs;
use std::str::FromStr;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t = String::from("example_nums.txt"))]
    filename: String
}
fn main() {
    let args = Args::parse();

    let filename = args.filename;
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("loaded the data");
    let nums:Vec<&str> = contents.split("\n").collect();
    let mut increases:i32 = 0; 
    let mut averages:Vec<i32> = vec![];
    for index in 2..nums.len() {
        averages.push(get_3_depth([&nums[index -2], &nums[index - 1], &nums[index]]))
    }
    for index in 3..nums.len() {
        if averages[index - 2] > averages[index - 3] {
            increases += 1;
            //print where increases have been found. Uncomment for debugging
            println!("found increase on index {}, average 1: {}, average 2: {}", index, averages[index -2], averages[index -3]);
        } else {
            println!("didn't find increase on index {}, average 1: {}, average 2: {}", index, averages[index -2], averages[index -3]);
        }
    }
    println!("Number of increases: {}", increases)
}

fn get_3_depth(nums: [&str; 3]) -> i32 {
    let num_1:i32 = i32::from_str(&nums[0][0..nums[0].len() - 1]).unwrap();
    let num_2:i32 = i32::from_str(&nums[1][0..nums[1].len() - 1]).unwrap();
    let num_3:i32 = i32::from_str(&nums[2][0..nums[2].len() - 1]).unwrap();
    let result: i32 = num_1 + num_2 + num_3;
    return result;
}
