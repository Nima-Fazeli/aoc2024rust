// day 3 challenge
use std::fs;

fn read_and_mul(path: &str) -> i32 {
    // let mul_arr: String = fs::read_to_string(&path)
    //     .expect("Should have been able to read the file");
    let with_verbose = false;

    let file_content: String = fs::read_to_string(&path)
        .expect("Could not read the file");
    let mul_arr : Vec<&str> = file_content.split("").collect();
    // let mul_arr: Vec<&str> = v.iter().map(|s| &**s).collect();

    let mut total = 0;
    // let mul_str = vec!["mul("];
    let mul_str : Vec<&str> = vec!["m", "u", "l", "("];
    let mul_break : Vec<&str> = vec![","];
    let mul_end : Vec<&str> = vec![")"];
    let number_map = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    if with_verbose {
        println!("mul break is {}", mul_break[0]);
        println!("mul end is {}", mul_end[0]);
        println!("------------");

        println!("{}", mul_arr[9]);
        if mul_arr[9] == mul_break[0]{
            println!("Found a break at 9");
        }
    }

    for i in 0..mul_arr.len()-5 {
        if with_verbose {
            println!("Index: {}, mul array: {}, mul str: {:?}", i, mul_arr[i], mul_str);
        }
        if mul_arr[i..i+4] == mul_str {
            if with_verbose {
                println!("\n\n ---- ");
                println!("Found a match at { }", i);
            }
            let psi = i + 4;
            let mut left = 0;
            let mut right = 0;
            let mut is_left = true;
            let mut is_number;

            // fill left first
            for count in psi..(psi + 9) {
                if with_verbose {
                    println!("count is {} and the value is {}", count, mul_arr[count]);
                }
                // the trouble is when parse::<i32>() is called on a comma or a closing parenthesis it panics
                // we need to check if the value is a number before we parse it
                match mul_arr[count].parse::<i32>() {
                    Ok(_) => is_number = true,
                    Err(_) => is_number = false,
                }

                if is_number == true {
                    if with_verbose {
                        println!("is {} in the hashmap? {}", mul_arr[count], number_map.contains(&mul_arr[count].parse::<i32>().unwrap()));
                    }
                    if number_map.contains(&mul_arr[count].parse::<i32>().unwrap()) {
                        if is_left == true {
                            left = left * 10 + mul_arr[count].parse::<i32>().unwrap();
                        } else {
                            right = right * 10 + mul_arr[count].parse::<i32>().unwrap();
                        }
                        if with_verbose {
                            println!("current left and right are {} {}", left, right);
                        }
                    }
                } else if mul_arr[count] == mul_break[0] {
                    if with_verbose {
                        println!("Found a break at {}", count);
                    }
                    is_left = false;
                } else if mul_arr[count] == mul_end[0] {
                    if with_verbose {
                        println!("Found an end at {}", count);
                    }
                    total = total + left * right;
                    is_left = true;
                    left = 0;
                    right = 0;
                    break;
                } else {
                    is_left = true;
                    left = 0;
                    right = 0;
                    break;
                }
            }
        }
    }
    return total
}

fn main() {
    let product = read_and_mul("/Users/nimafazeli/Documents/virtual-envs/rust-code/aoc2024/src/day3code.txt");
    println!("The product is: {}", product);
}
