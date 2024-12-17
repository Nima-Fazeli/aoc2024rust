// day 3 challenge
use std::fs;

fn read_and_mul(path: &str) -> i32 {
    let file_content: String = fs::read_to_string(&path)
        .expect("Could not read the file");
    let lines : Vec<&str> = file_content.lines().collect();

    let mut total = 0;
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let target = "XMAS";
    let target_rev = "SAMX";

    // horizontal count
    for line in &lines {
        for i in 0..(line.len() - 3) {
            if &line[i..(i + 4)] == target || &line[i..(i+4)] == target_rev {
                total = total + 1;
            }
        }
    }

    println!("Horizontal count: {}", total);

    // count vertical
    for row in 0..(height - 3){
        for col in 0..width {
            let mut vertical_str = String::new();

            for i in 0..4 {
                vertical_str.push(lines[row + i].chars().nth(col).unwrap());
            }

            if vertical_str == target || vertical_str == target_rev {
                total = total + 1;
            }
        }
    }

    println!("Vertical plus horizontal count: {}", total);

    // count diagonal right
    for row in 0..(height - 3){
        for col in 0..(width - 3) {
            let mut diagonal_str = String::new();

            for i in 0..4 {
                diagonal_str.push(lines[row + i].chars().nth(col + i).unwrap());
            }

            if diagonal_str == target || diagonal_str == target_rev {
                total = total + 1;
            }
        }
    }

    println!("(h + v + dr) = {}", total);

    // count diagonal left
    for row in 0..(height - 3){
        for col in 3..width {
            let mut diagonal_str = String::new();

            for i in 0..4 {
                diagonal_str.push(lines[row + i].chars().nth(col - i).unwrap());
            }

            if diagonal_str == target || diagonal_str == target_rev {
                total = total + 1;
            }
        }
    }

    return total;
}

fn main() {
    let total = read_and_mul("/Users/nimafazeli/Documents/virtual-envs/rust-code/aoc2024/src/day4code.txt");
    println!("The total count of XMAS: {}", total);
}
