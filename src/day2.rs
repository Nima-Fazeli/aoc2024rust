// day 1 challenge

use std::fs;

// one function to take the difference and check all elements
// if all elements have the same sign and no element is greater than zero
// than its ascending/descending and valid
fn is_valid(arr: &[i32]) -> bool {
    // first compute the difference
    let mut count : usize = 0;
    for i in 0..arr.len() {
        if arr[i] > 0 {
            count = count + 1;
        }
    }
    
    count = count - 1;
    let mut diff_arr = vec![0; count];
    for i in 0..count {
        diff_arr[i] = arr[i + 1] - arr[i];
    }
    
    // check to see if we have any sign changes w.r.t. first element or any value
    // larger than 3
    let mut is_valid: bool = true;
    
    if diff_arr[0] == 0 || ((diff_arr[0]).abs() > 3) {
        is_valid = false;
    }

    if diff_arr[0] > 0 {
        for i in 1..count {
            // check to see if each value is positive
            if (diff_arr[i] <= 0) || ( (diff_arr[i]).abs() > 3) {
                is_valid = false;
            }
        }
    }
    else if diff_arr[0] < 0 {
        for i in 1..count {
            if (diff_arr[i] >= 0) || ((diff_arr[i]).abs() > 3) {
                is_valid = false;
            }
        }
    }
    return is_valid;
}

// read file and run each check above
fn read_and_check(path: &str) -> i32 {
    let file_content: String = fs::read_to_string(&path)
        .expect("Should have been able to read the file");

    // returns an iterator over lines of a string
    let v: Vec<&str> = file_content.split('\n').collect();

    let mut count : i32 = 0;
    let mut is_safe : bool;

    for i in 0..v.len() - 1 {
        
        let p: Vec<_> = v[i].split_whitespace().collect();
        let q: Vec<_> = p.into_iter().map(|w| w.parse::<i32>().unwrap()).collect();
        is_safe = is_valid(&q);

        if i < 10 {
            println!("i = { }, v = { }, is_safe = { }", i, v[i], is_safe);
        }

        if is_safe == true { 
            count = count + 1;
        }
    }

    println!("I counted { } safe codes", count);
    return count;
}

fn main() {
    let _count = read_and_check("/Users/nimafazeli/Documents/virtual-envs/rust-code/aoc2024/src/day2code.txt");
}
