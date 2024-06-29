use std::cmp::Ordering;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split('\n').collect();

    // part 1
    let rounds1: Vec<(Rps, Rps)> = input
        .iter()
        .map(|x| {
            let mut split = x.split(' ');
            (
                Rps::from(split.next().expect("Need valid move!")),
                Rps::from(split.next().expect("Need valid move!")),
            )
        })
        .collect();

    let part1: usize = rounds1.iter().map(score).sum();
    println!("The total score would be {}", part1);

    // part 2
    let rounds2: Vec<(Rps, Rps)> = input
        .iter()
        .map(|x| {
            let mut split = x.split(' ');
            let other = Rps::from(split.next().expect("Need valid move!"));
            let strategy = split.next().expect("Need a valid strategy!");
            let my_move = match strategy {
                "X" => other.lose(),
                "Y" => other.clone(),
                "Z" => other.defeat(),
                _ => panic!("{} is not a valid strategy!", strategy),
            };
            (other, my_move)
        })
        .collect();

    let part2: usize = rounds2.iter().map(score).sum();
    println!("The total score would be {}", part2);
}

fn score(game: &(Rps, Rps)) -> usize {
    let mut s = match game.1 {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    };
    s += if game.0 < game.1 {
        6
    } else if game.0 == game.1 {
        3
    } else {
        0
    };
    s
}

#[derive(PartialEq, Debug, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("{} is not a valid move!", s),
        }
    }

    fn defeat(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn lose(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Rock => match other {
                Self::Rock => Some(Ordering::Equal),
                Self::Paper => Some(Ordering::Less),
                Self::Scissors => Some(Ordering::Greater),
            },
            Self::Paper => match other {
                Self::Rock => Some(Ordering::Greater),
                Self::Paper => Some(Ordering::Equal),
                Self::Scissors => Some(Ordering::Less),
            },
            Self::Scissors => match other {
                Self::Rock => Some(Ordering::Less),
                Self::Paper => Some(Ordering::Greater),
                Self::Scissors => Some(Ordering::Equal),
            },
        }
    }
}
