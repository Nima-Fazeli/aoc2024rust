// day 5 challenge
use std::fs;
use std::collections::HashMap;

fn create_hash(path: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let file_content: String = fs::read_to_string(&path)
        .expect("Could not read the file");

    let file_split : Vec<&str> = file_content.split("\n\n").collect();

    let instructions : Vec<&str> = file_split[0].split("\n").collect();

    let codes : Vec<&str> = file_split[1].split("\n").collect();
    let codes : Vec<String> = codes.iter().map(|x| x.to_string()).collect();

    let number_of_instructions = instructions.len();

    // this part is wrong -- each instruction can have multiple conditions
    // a hashmap does not work because its a one to one mapping
    // we need to use a vector of vectors
    let mut instruct : Vec<Vec<String>> = Vec::new();

    for i in 0..(number_of_instructions - 1) {
        let split = instructions[i].split('|').collect::<Vec<&str>>();
        instruct.push(vec![split[0].to_string(), split[1].to_string()]);
    }

    /*
    println!("Here are the first 10 intructions");
    for i in 0..10 {
        println!("{:?}", instruct[i]);
    }

    println!("Here are the first 10 codes");
    for i in 0..10 {
        println!("{}", codes[i]);
    }
    */
    return (instruct, codes);
}

fn count_valid_codes(instruct: Vec<Vec<String>>, codes: Vec<String>) -> i32 {
    let mut is_okay = true;
    let mut count = 0;
    let mut sum = 0;

    for line in 0..(codes.len() - 1) {
        let test = codes[line].split(',').collect::<Vec<&str>>();
        is_okay = true;

        for i in 1..test.len() {
            let mut applicable_instructions = Vec::new();
            for j in 0..instruct.len() {
                if instruct[j].contains(&test[i].to_string()) {
                    applicable_instructions.push(j);
                }
            }
            if is_okay == true {
                for entry in 0..i {
                    for instruction_id in 0..applicable_instructions.len() {
                        if test[entry] == instruct[applicable_instructions[instruction_id]][1] {
                            is_okay = false;
                            break;
                        }
                    }
                }
            }
        }

        if is_okay == true {
            sum = sum + test[(test.len() + 1) / 2 - 1].parse::<i32>().unwrap();
            count = count + 1;
        }
    }

    println!("The number of valid codes is {}", count);
    println!("The total sum is {}", sum);
    return sum;
}

fn main() {
    let (instruct, codes) = create_hash("/Users/nimafazeli/Documents/virtual-envs/rust-code/aoc2024/src/day5code.txt");
    let _valid = count_valid_codes(instruct, codes);
}
