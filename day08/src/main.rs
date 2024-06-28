use std::fs;
// use std::Iterator::rev;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split("\n").collect();

    let trees: Vec<Vec<i8>> = input
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).expect("Should be valid digit!") as i8)
                .collect()
        })
        .collect();
    let height = trees.len();
    let width = trees[0].len();

    // part 1
    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    // view from left
    for y in 0..height {
        let mut highest = -1_i8;
        for x in 0..width {
            if trees[y][x] > highest {
                visible[y][x] = true;
                highest = trees[y][x];
            }
        }
    }
    // view from right
    for y in 0..height {
        let mut highest = -1_i8;
        for x in (0..width).rev() {
            if trees[y][x] > highest {
                visible[y][x] = true;
                highest = trees[y][x];
            }
        }
    }
    // view from top
    for x in 0..width {
        let mut highest = -1_i8;
        for y in 0..height {
            if trees[y][x] > highest {
                visible[y][x] = true;
                highest = trees[y][x];
            }
        }
    }
    // view from bottom
    for x in 0..width {
        let mut highest = -1_i8;
        for y in (0..height).rev() {
            if trees[y][x] > highest {
                visible[y][x] = true;
                highest = trees[y][x];
            }
        }
    }
    let total_visible: usize = visible
        .iter()
        .map(|row| row.iter().filter(|x| x == &&true).count())
        .sum();
    let part1 = total_visible;
    println!("{} trees are visible from outside the grid", part1);

    // part 2
    let mut scenic_score: Vec<Vec<u32>> = vec![vec![0; width]; height];
    for y in 0..height {
        for x in 0..width {
            let tree_height = trees[y][x];

            // look left
            let mut left = 0;
            let mut new_x = x;
            while new_x > 0 {
                new_x -= 1;
                left += 1;
                if trees[y][new_x] >= tree_height {
                    break;
                }
            }
            // look right
            let mut right = 0;
            let mut new_x = x;
            while new_x < (width - 1) {
                new_x += 1;
                right += 1;
                if trees[y][new_x] >= tree_height {
                    break;
                }
            }
            // look up
            let mut up = 0;
            let mut new_y = y;
            while new_y > 0 {
                new_y -= 1;
                up += 1;
                if trees[new_y][x] >= tree_height {
                    break;
                }
            }
            // look down
            let mut down = 0;
            let mut new_y = y;
            while new_y < (height - 1) {
                new_y += 1;
                down += 1;
                if trees[new_y][x] >= tree_height {
                    break;
                }
            }
            scenic_score[y][x] = left * right * up * down;
        }
    }
    let max_scenic_score = scenic_score
        .iter()
        .map(|x| x.iter().max().expect("Should be valid number!"))
        .max()
        .expect("Should be valid number!");
    let part2 = max_scenic_score;
    println!(
        "The highest scenic score possible for any tree is {}",
        part2
    );
}
