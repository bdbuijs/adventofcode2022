use std::collections::VecDeque;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input: Vec<&str> = input_file.split("\n\n").collect();

    // part 1
    let mut monkeys = Troop::new();
    for m in input.iter() {
        let monkey = Monkey::from(*m);
        monkeys.add(monkey);
    }
    for _ in 0..20 {
        monkeys.round(false);
    }
    let part1 = monkeys.monkey_business();
    println!(
        "The level of monkey business after 20 rounds of stuff-slinging simian shenanigans is {}",
        part1
    );

    // part 2
    let mut monkeys = Troop::new();
    for m in input.iter() {
        let monkey = Monkey::from(*m);
        monkeys.add(monkey);
    }
    for _ in 0..10_000 {
        monkeys.round(true);
    }
    let part2 = monkeys.monkey_business();
    println!(
        "The level of monkey business after 10000 rounds is {}",
        part2
    );
}

struct Operation {
    signature: String,
    func: Box<dyn Fn(usize) -> usize>,
}

impl Operation {
    fn run(&self, x: usize) -> usize {
        (self.func)(x)
    }
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.signature)
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let mut split = s.split(' ');
        let operand1 = split.next().expect("must be valid operand");
        let symbol = split.next().expect("must be valid operand");
        let operand2 = split.next().expect("must be valid operand");
        if operand1 == operand2 {
            Self {
                signature: s.to_owned(),
                func: Box::new(|x| x * x),
            }
        } else {
            let right = operand2.parse::<usize>().expect("Should be valid operand");
            if symbol == "*" {
                Self {
                    signature: s.to_owned(),
                    func: Box::new(move |x| x * right),
                }
            } else {
                Self {
                    signature: s.to_owned(),
                    func: Box::new(move |x| x + right),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    index: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test_mod: usize,
    throw_true: usize,
    throw_false: usize,
    inspect_count: usize,
}

impl Monkey {
    fn go(&mut self, worried: bool, troop_mod: usize) -> Option<(usize, usize)> {
        if let Some(item) = self.items.pop_front() {
            let worry_level = match worried {
                true => self.operation.run(item) % troop_mod + troop_mod,
                false => self.operation.run(item) / 3,
            };
            let next_monkey = match worry_level % self.test_mod {
                0 => self.throw_true,
                _ => self.throw_false,
            };
            self.inspect_count += 1;
            Some((worry_level, next_monkey))
        } else {
            None
        }
    }

    fn throw(&mut self, value: usize) {
        self.items.push_back(value);
    }

    fn count(&self) -> usize {
        self.inspect_count
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut split = s.split('\n');
        let index = split
            .next()
            .expect("Must be valid monkey!")
            .strip_prefix("Monkey ")
            .expect("Must be valid monkey!")
            .strip_suffix(':')
            .expect("Must be valid monkey!")
            .parse::<usize>()
            .expect("Item must have a valid number!");
        let items: VecDeque<usize> = split
            .next()
            .expect("Must be valid monkey!")
            .strip_prefix("  Starting items: ")
            .expect("Must be valid monkey!")
            .split(", ")
            .map(|x| x.parse::<usize>().expect("Must be valid monkey!"))
            .collect();
        let op = split
            .next()
            .expect("Must be valid monkey!")
            .strip_prefix("  Operation: new = ")
            .expect("Must be valid monkey!");
        let operation = Operation::from(op);
        let test_mod = split
            .next()
            .expect("Must be valid monkey!")
            .strip_prefix("  Test: divisible by ")
            .expect("Must be valid monkey!")
            .parse::<usize>()
            .expect("Item must have a valid number!");
        let throw_true = split
            .next()
            .expect("Must be valid monkey!")
            .strip_prefix("    If true: throw to monkey ")
            .expect("Must be valid monkey!")
            .parse::<usize>()
            .expect("Item must have a valid number!");
        let throw_false = split
            .next()
            .expect("Must be valid monkey!")
            .strip_prefix("    If false: throw to monkey ")
            .expect("Must be valid monkey!")
            .parse::<usize>()
            .expect("Item must have a valid number!");
        Self {
            index,
            items,
            operation,
            test_mod,
            throw_true,
            throw_false,
            inspect_count: 0,
        }
    }
}

struct Troop {
    monkeys: Vec<Monkey>,
    turn: usize,
    troop_mod: usize,
}

impl Troop {
    fn new() -> Self {
        Self {
            monkeys: Vec::new(),
            turn: 0,
            troop_mod: 1,
        }
    }

    fn add(&mut self, monkey: Monkey) {
        self.troop_mod *= monkey.test_mod;
        self.monkeys.push(monkey);
    }

    fn throw(&mut self, monkey_index: usize, item: usize) {
        self.monkeys
            .get_mut(monkey_index)
            .expect("Monkey should exist")
            .throw(item);
    }

    fn go(&mut self, monkey_index: usize, worried: bool) -> Option<(usize, usize)> {
        let troop_mod = self.troop_mod;
        self.monkeys.get_mut(monkey_index)?.go(worried, troop_mod)
    }

    fn round(&mut self, worried: bool) {
        while self.turn < self.monkeys.len() {
            let mut deque = VecDeque::new();
            while let Some(tup) = self.go(self.turn, worried) {
                deque.push_back(tup)
            }
            while let Some((item, next_monkey)) = deque.pop_front() {
                self.throw(next_monkey, item);
            }
            self.turn += 1;
        }
        self.turn = 0;
    }

    fn monkey_business(&self) -> usize {
        let mut inspections: Vec<usize> =
            self.monkeys.iter().map(|monkey| monkey.count()).collect();
        inspections.sort_unstable();

        inspections.pop().expect("Must have two") * inspections.pop().expect("Must have two")
    }
}

impl std::fmt::Debug for Troop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for monkey in self.monkeys.iter() {
            s.extend(format!("Monkey {}: ", monkey.index).chars());
            for item in monkey.items.iter() {
                s.push_str(item.to_string().as_str());
                s.push(',');
            }
            s.push('\n')
        }
        f.write_str(s.as_str())
    }
}
