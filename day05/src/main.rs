use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split("\n\n").collect();
    let stacks_strings: Vec<&str> = input[0].split("\n").collect();
    let moves: Vec<&str> = input[1].split("\n").collect();
    let mut stacks = vec![vec![]];
    for st in stacks_strings.into_iter() {
        stacks.push(Vec::from_iter(st.chars()))
    }

    // part1
    let mut stacks1 = stacks.clone();
    for mv in moves.iter() {
        let mut spl = mv.split(" ");
        spl.next();
        let amount = spl
            .next()
            .expect("Should not be empty!")
            .parse::<usize>()
            .expect("Should be valid number!");
        spl.next();
        let from = spl
            .next()
            .expect("Should not be empty!")
            .parse::<usize>()
            .expect("Should be valid number!");
        spl.next();
        let to = spl
            .next()
            .expect("Should not be empty!")
            .parse::<usize>()
            .expect("Should be valid number!");
        for _ in 0..amount {
            let to_move = stacks1[from].pop().expect("Should not be empty!");
            stacks1[to].push(to_move);
        }
    }
    let mut tops1: String = "".to_string();
    for mut stack in stacks1 {
        if stack.len() > 0 {
            tops1.push(stack.pop().expect("Should not be empty!"));
        }
    }
    let part1 = tops1;
    println!("The top crates of each stack are: {}", part1);

    // part 2
    let mut stacks2 = stacks.clone();

    for mv in moves.iter() {
        let mut spl = mv.split(" ");
        spl.next();
        let amount = spl
            .next()
            .expect("Should not be empty!")
            .parse::<usize>()
            .expect("Should be valid number!");
        spl.next();
        let from = spl
            .next()
            .expect("Should not be empty!")
            .parse::<usize>()
            .expect("Should be valid number!");
        spl.next();
        let to = spl
            .next()
            .expect("Should not be empty!")
            .parse::<usize>()
            .expect("Should be valid number!");
        let mut temp_stack = vec![];
        for _ in 0..amount {
            temp_stack.push(stacks2[from].pop().expect("Should not be empty!"));
        }
        while temp_stack.len() > 0 {
            stacks2[to].push(temp_stack.pop().expect("Should not be empty!"))
        }
    }
    let mut tops2: String = "".to_string();
    for mut stack in stacks2 {
        if stack.len() > 0 {
            tops2.push(stack.pop().expect("Should not be empty!"));
        }
    }
    let part2 = tops2;
    println!("Now the top crates of each stack are: {}", part2);
}
