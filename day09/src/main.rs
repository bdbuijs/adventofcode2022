use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split('\n').collect();
    let moves: Vec<(&str, u32)> = input
        .iter()
        .map(|x| {
            let mut it = x.split(' ');
            let mv = it.next().expect("Should have valid move!");
            let amount = it
                .next()
                .expect("Should have valid amount!")
                .parse::<u32>()
                .expect("Should have valid amount!");
            (mv, amount)
        })
        .collect();

    // part 1
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for (mv, amount) in moves.iter() {
        match *mv {
            "U" => {
                for _ in 0..*amount {
                    head.1 += 1;
                    follow_knot(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            "D" => {
                for _ in 0..*amount {
                    head.1 -= 1;
                    follow_knot(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            "L" => {
                for _ in 0..*amount {
                    head.0 -= 1;
                    follow_knot(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            "R" => {
                for _ in 0..*amount {
                    head.0 += 1;
                    follow_knot(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            _ => panic!("Should have valid move!"),
        }
    }
    let part1 = visited.len();
    println!(
        "The tail of the rope visits {} positions at least once",
        part1
    );

    // part 2
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut visited2: HashSet<(i32, i32)> = HashSet::new();

    for (mv, amount) in moves.iter() {
        match *mv {
            "U" => {
                for _ in 0..*amount {
                    rope[0].1 += 1;
                    for i in 1..10 {
                        let head = rope[i - 1];
                        let tail = &mut rope[i];
                        follow_knot(&head, tail);
                    }
                    visited2.insert(rope[9]);
                }
            }
            "D" => {
                for _ in 0..*amount {
                    rope[0].1 -= 1;
                    for i in 1..10 {
                        let head = rope[i - 1];
                        let tail = &mut rope[i];
                        follow_knot(&head, tail);
                    }
                    visited2.insert(rope[9]);
                }
            }
            "L" => {
                for _ in 0..*amount {
                    rope[0].0 -= 1;
                    for i in 1..10 {
                        let head = rope[i - 1];
                        let tail = &mut rope[i];
                        follow_knot(&head, tail);
                    }
                    visited2.insert(rope[9]);
                }
            }
            "R" => {
                for _ in 0..*amount {
                    rope[0].0 += 1;
                    for i in 1..10 {
                        let head = rope[i - 1];
                        let tail = &mut rope[i];
                        follow_knot(&head, tail);
                    }
                    visited2.insert(rope[9]);
                }
            }
            _ => panic!("Should have valid move!"),
        }
    }
    let part2 = visited2.len();
    println!(
        "The tail of the longer rope visits {} positions at least once",
        part2
    );
}

fn follow_knot(head: &(i32, i32), tail: &mut (i32, i32)) {
    let y = head.1 - tail.1;
    let x = head.0 - tail.0;
    match x {
        -2 => match y {
            -2 => {
                tail.0 -= 1;
                tail.1 -= 1;
            }
            -1 => {
                tail.0 -= 1;
                tail.1 -= 1;
            }
            0 => {
                tail.0 -= 1;
            }
            1 => {
                tail.0 -= 1;
                tail.1 += 1;
            }
            2 => {
                tail.0 -= 1;
                tail.1 += 1;
            }
            _ => panic!("Can't have this difference!"),
        },
        -1 => match y {
            -2 => {
                tail.1 -= 1;
                tail.0 -= 1;
            }
            -1 => {}
            0 => {}
            1 => {}
            2 => {
                tail.1 += 1;
                tail.0 -= 1;
            }
            _ => panic!("Can't have this difference!"),
        },
        0 => match y {
            -2 => {
                tail.1 -= 1;
            }
            -1 => {}
            0 => {}
            1 => {}
            2 => tail.1 += 1,
            _ => panic!("Can't have this difference!"),
        },
        1 => match y {
            -2 => {
                tail.1 -= 1;
                tail.0 += 1;
            }
            -1 => {}
            0 => {}
            1 => {}
            2 => {
                tail.1 += 1;
                tail.0 += 1;
            }
            _ => panic!("Can't have this difference!"),
        },
        2 => match y {
            -2 => {
                tail.0 += 1;
                tail.1 -= 1;
            }
            -1 => {
                tail.0 += 1;
                tail.1 -= 1;
            }
            0 => {
                tail.0 += 1;
            }
            1 => {
                tail.0 += 1;
                tail.1 += 1;
            }
            2 => {
                tail.0 += 1;
                tail.1 += 1;
            }
            _ => panic!("Should have valid move!"),
        },
        _ => panic!("Can't have this difference!"),
    }
}
