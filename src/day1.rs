// day 1 challenge

use std::fs;

fn read(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file_content: String = fs::read_to_string(&path)
        .expect("Should have been able to read the file");

    let v: Vec<&str> = file_content.split(&['\n', ' '][..]).collect();
    

    let mut vec_l : Vec<i32> = Vec::new();
    let mut vec_r : Vec<i32> = Vec::new();
    

    for i in 0..v.len()-1 {
        if i < 16 {
            println!("i = { }, v = { }", i, v[i]);
        }
        if i % 4 == 0 {
            vec_l.push(v[i].parse::<i32>().unwrap());
        }
        if i % 4 == 3 {
            vec_r.push(v[i].parse::<i32>().unwrap());
        }
    }

    return (vec_l, vec_r);
}

fn sum_sorted(mut vec_r : Vec<i32>, mut vec_l : Vec<i32>) -> i32 {
    vec_l.sort();
    vec_r.sort();
    
    let mut sum: i32 = 0;
    for i in 0..vec_l.len() {
        if i < 10 {
            println!("left/right numbers are  ({ }, { })", vec_l[i], vec_r[i]);
            println!("Abs diff is { }", (vec_l[i] - vec_r[i]).abs());
        }
        sum = sum + (vec_l[i] - vec_r[i]).abs()    
    }

    println!("total sum is { }", sum);
    
    return sum;
}

fn main() {
    let (vl, vr) = read("/Users/nimafazeli/Documents/virtual-envs/rust-code/aoc2024/src/day1code.txt");
    let _sum = sum_sorted(vr, vl);
}
