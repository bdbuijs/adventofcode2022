use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split('\n').collect();

    // part 1
    let mut total_priority = 0;
    for rucksack in input.iter() {
        let mid = rucksack.len() / 2;
        let compartment_1: HashSet<char> = HashSet::from_iter(rucksack[0..mid].chars());
        let compartment_2: HashSet<char> = HashSet::from_iter(rucksack[mid..].chars());
        let overlap = compartment_1
            .intersection(&compartment_2)
            .last()
            .expect("Should contain a character!");
        let mut priority = 0;
        if overlap.is_uppercase() {
            let lowercase = overlap
                .to_lowercase()
                .next()
                .expect("Should contain a character!");
            priority += lowercase.to_digit(36).expect("Should contain a character!") + 17;
        } else {
            priority += overlap.to_digit(36).expect("Should contain a character!") - 9;
        }
        total_priority += priority;
    }

    let part1 = total_priority;
    println!(
        "The sum of the priorities of the overlapping item types is {}",
        part1
    );

    // part 2
    let mut total_badges = 0;
    for group in input.chunks(3) {
        let mut elf1: HashSet<char> = HashSet::from_iter(group[0].chars());
        let elf2: HashSet<char> = HashSet::from_iter(group[1].chars());
        let elf3: HashSet<char> = HashSet::from_iter(group[2].chars());
        elf1.retain(|x| elf2.contains(x));
        elf1.retain(|x| elf3.contains(x));

        let overlap = elf1
            .intersection(&elf1)
            .last()
            .expect("Should contain a character!");
        let mut priority = 0;
        if overlap.is_uppercase() {
            let lowercase = overlap
                .to_lowercase()
                .next()
                .expect("Should contain a character!");
            priority += lowercase.to_digit(36).expect("Should contain a character!") + 17;
        } else {
            priority += overlap.to_digit(36).expect("Should contain a character!") - 9;
        }
        total_badges += priority;
    }
    let part2 = total_badges;
    println!("The sum of the priorities of the badges is {}", part2);
}
