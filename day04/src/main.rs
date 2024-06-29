use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split('\n').collect();

    // part 1 & 2
    let mut contained = 0;
    let mut overlap = 0;
    for row in input.iter() {
        let elves: Vec<Vec<usize>> = row
            .split(',')
            .map(|x| {
                x.split('-')
                    .map(|y| y.parse::<usize>().expect("Must be valid number!"))
                    .collect()
            })
            .collect();
        let elf1: HashSet<usize> = HashSet::from_iter(elves[0][0]..=elves[0][1]);
        let elf2: HashSet<usize> = HashSet::from_iter(elves[1][0]..=elves[1][1]);

        if elf1.is_disjoint(&elf2) {
            continue;
        }
        overlap += 1;

        if elf1.is_subset(&elf2) || elf2.is_subset(&elf1) {
            contained += 1;
        }
    }
    let part1 = contained;
    println!("In {} pairs the one range fully contains the other", part1);

    let part2 = overlap;
    println!("In {} pairs the ranges overlap", part2);
}
