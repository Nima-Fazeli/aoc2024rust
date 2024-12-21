// day 6 challenge
use std::fs;

fn create_grid(path: &str) -> i32 {
    let file_content: String = fs::read_to_string(&path)
        .expect("Could not read the file");

    let grid : Vec<&str> = file_content.lines().collect();
    let height = grid.len();
    let width = grid.first().unwrap().len();

    let character_direction : Vec<char> = vec!['<', '^', '>', 'v'];
    let mut visited_locations : Vec<(i32, i32)> = Vec::new();
    let mut current_direction = 0 as usize;

    let direction_to_move : Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut candidate_next_x : i32;
    let mut candidate_next_y : i32;
    let mut current_x : i32;
    let mut current_y : i32;

    for i in 0..width {
        for j in 0..height {
            if grid[j].chars().nth(i).unwrap() == character_direction[0] {
                visited_locations.push((i as i32, j as i32));
                current_direction = 0 as usize;
            }
            else if grid[j].chars().nth(i).unwrap() == character_direction[1] {
                visited_locations.push((i as i32, j as i32));
                current_direction = 1 as usize;
            }
            else if grid[j].chars().nth(i).unwrap() == character_direction[2] {
                visited_locations.push((i as i32, j as i32));
                current_direction = 2 as usize;
            }
            else if grid[j].chars().nth(i).unwrap() == character_direction[3] {
                visited_locations.push((i as i32, j as i32));
                current_direction = 3 as usize;
            }
        }
    }

    let with_verbose = false;
    let mut while_flag = true;
    current_x = visited_locations.last().unwrap().0;
    current_y = visited_locations.last().unwrap().1;

    println!(" ---- \n");
    while while_flag {
        println!("current unique count is: {}", visited_locations.len());
        if with_verbose {println!("Visited locations: {:?} and the current direction is {:?}", visited_locations, current_direction)};

        // things to check for: first, check the direction. Second, figure out which bin to query.
        // Third, check if the bin has # or not. Finally, make sure that if you move, you stay in the grid.

        if with_verbose {println!("visited location last is {:?}", visited_locations.last().unwrap())};
        if with_verbose {println!("Direction to move {:?}", direction_to_move[current_direction])};
        candidate_next_x = direction_to_move[current_direction].0 + current_x;
        candidate_next_y = direction_to_move[current_direction].1 + current_y;
        if with_verbose {println!("Given my current direction, I need to move ({:?}, {:?})", candidate_next_x, candidate_next_y)};

        if candidate_next_x < 0 || candidate_next_x >= width as i32 || candidate_next_y < 0 || candidate_next_y >= height as i32 {
            if with_verbose {println!("I am out of the grid")};
            while_flag = false;
            break;
        }
        else
        if grid[candidate_next_y as usize].chars().nth(candidate_next_x as usize).unwrap() == '#' {
            current_direction = (current_direction + 1) % 4;
            if with_verbose {println!("I am in a tree, need to turn right which means {:?}", current_direction)};
            current_x = direction_to_move[current_direction].0 + current_x;
            current_y = direction_to_move[current_direction].1 + current_y;
        }
        else {
            if with_verbose {println!("I am not in a tree")};
            current_x = candidate_next_x;
            current_y = candidate_next_y;
        }
        // check to see if the next location is already visited
        if visited_locations.contains(&(current_x, current_y)) {
            if with_verbose {println!("I have already visited this location")};
        } else {
            if with_verbose {println!("I have not visited this location")};
            visited_locations.push((current_x, current_y));
        }
    }
    println!("The number of unique visited locations is {}", visited_locations.len());
    return visited_locations.len() as i32;
}

fn main() {
    let _unique_visits = create_grid("/Users/nimafazeli/Documents/virtual-envs/rust-code/aoc2024/src/day6code.txt");
}
