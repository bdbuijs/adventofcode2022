use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split("\n").collect();

    // part 1 & 2
    let mut cycle = 1;
    let mut register_x = 1;
    let mut total = 0;
    print!(" ");
    for instruction in input.iter() {
        match instruction {
            &"noop" => {
                cycle += 1;
                total += check(&cycle, &register_x);
            }
            _ => {
                let num = instruction
                    .split(" ")
                    .skip(1)
                    .next()
                    .expect("Should be valid number!")
                    .parse::<i32>()
                    .expect("Should be valid number!");
                cycle += 1;
                total += check(&cycle, &register_x);
                register_x += num;
                cycle += 1;
                total += check(&cycle, &register_x);
            }
        }
    }
    let part1 = total;
    println!("The sum of the six signal strengths is {}", part1);
}

fn check(cycle: &i32, register: &i32) -> i32 {
    let position = (cycle - 1) % 40;
    match position - register {
        -1..=1 => print!("#"),
        _ => print!(" "),
    };
    if position == 39 {
        print!("\n");
    }
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => cycle * register,
        _ => 0,
    }
}
