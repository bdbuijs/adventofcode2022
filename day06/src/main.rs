use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<char> = input_file
        .split('\n')
        .next()
        .expect("Should have valid string!")
        .chars()
        .collect();

    // part 1
    let mut position4 = 0;
    for i in 3..input.len() {
        let s = HashSet::<&char>::from_iter(input[(i - 3)..=i].iter());
        if s.len() == 4 {
            position4 = i + 1;
            break;
        }
    }
    let part1 = position4;
    println!(
        "{} characters need to be processed before the first start-of-packet marker is detected",
        part1
    );

    // part 2
    let mut position14 = 0;
    for i in 13..input.len() {
        let s = HashSet::<&char>::from_iter(input[(i - 13)..=i].iter());
        if s.len() == 14 {
            position14 = i + 1;
            break;
        }
    }
    let part2 = position14;
    println!(
        "{} characters need to be processed before the first start-of-message marker is detected",
        part2
    );
}
