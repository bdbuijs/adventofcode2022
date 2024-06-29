use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split('\n').collect();

    // part 1
    let part1 = 1;
    println!("{}", part1);

    // part 2
    let part2 = 2;
    println!("{}", part2);
}
