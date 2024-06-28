use itertools::Itertools;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split("\n\n").collect();

    let elves: Vec<usize> = // sorted with largest calorie count last
        input.iter()
            .map(|x| x.split("\n")
                .map(|y| y.parse::<usize>()
                    .expect("Need valid number!"))
                .sum())
            .sorted()
            .collect();

    let end = elves.len() - 1;

    // part 1
    let part1 = elves[end];
    println!(
        "The Elf carrying the most Calories is carrying {} Calories in total",
        part1
    );

    // part 2
    let part2 = elves[end] + elves[end - 1] + elves[end - 2];
    println!(
        "The top three Elves carrying the most Calories are carrying {} Calories in total",
        part2
    );
}
