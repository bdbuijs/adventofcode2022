use nom::{
    character::complete::{i64 as nomi64, newline},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, original_nums) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut linked_list = original_nums.into_iter().map(Node::new).collect::<Vec<_>>();
    let width = linked_list.len();
    (0..width).for_each(|i| {
        let left = (i + width - 1) % width;
        let right = (i + 1) % width;
        linked_list[i].left = left;
        linked_list[i].right = right;
    });
    debug_assert!(linked_list
        .iter()
        .all(|node| node.left < width && node.right < width));

    mix(&mut linked_list);
    let mut pos = linked_list
        .iter()
        .position(|node| node.value == 0)
        .expect("There's always a zero");
    let mut sum = 0;
    (0..3).for_each(|_| {
        (0..1000).for_each(|_| pos = linked_list[pos].right);
        sum += linked_list[pos].value;
    });

    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let decryption_key = 811589153_i64;
    let (input, original_nums) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut linked_list = original_nums.into_iter().map(Node::new).collect::<Vec<_>>();
    let width = linked_list.len();
    (0..width).for_each(|i| {
        let left = (i + width - 1) % width;
        let right = (i + 1) % width;
        linked_list[i].left = left;
        linked_list[i].right = right;
        linked_list[i].value *= decryption_key;
    });
    debug_assert!(linked_list
        .iter()
        .all(|node| node.left < width && node.right < width));

    (0..10).for_each(|_| mix(&mut linked_list));
    let mut pos = linked_list
        .iter()
        .position(|node| node.value == 0)
        .expect("There's always a zero");
    let mut sum = 0;
    (0..3).for_each(|_| {
        (0..1000).for_each(|_| pos = linked_list[pos].right);
        sum += linked_list[pos].value;
    });

    sum.to_string()
}

struct Node {
    value: i64,
    left: usize,
    right: usize,
}

impl Node {
    fn new(value: i64) -> Self {
        Self {
            value,
            left: usize::MAX,
            right: usize::MAX,
        }
    }
}

#[allow(dead_code)]
fn print_list(linked_list: &[Node]) {
    let mut pos = 0;
    (0..linked_list.len()).for_each(|_| {
        print!("{}, ", linked_list[pos].value);
        pos = linked_list[pos].right
    });
    println!("{}", linked_list[pos].value);
}

fn mix(linked_list: &mut [Node]) {
    let width = linked_list.len();
    (0..width).for_each(|i| {
        // print_list(&linked_list);
        if linked_list[i].value == 0 {
            return; // continue
        }
        let value = linked_list[i].value;
        let mut left = linked_list[i].left;
        let mut right = linked_list[i].right;
        // remove node
        linked_list[left].right = right;
        linked_list[right].left = left;
        // walk to insertion point
        match value.signum() {
            1 => {
                let steps = value as usize % (width - 1);
                (0..steps).for_each(|_| right = linked_list[right].right);
                // insert to the left!!!
                left = linked_list[right].left;
            }
            -1 => {
                let steps = value.unsigned_abs() as usize % (width - 1);
                (0..steps).for_each(|_| left = linked_list[left].left);
                // insert to the right!
                right = linked_list[left].right;
            }
            x => unreachable!("Impossible value for steps.signum(): {x}"),
        }
        // re-insert node
        linked_list[left].right = i;
        linked_list[right].left = i;
        linked_list[i].right = right;
        linked_list[i].left = left;
    });
}

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, lines) = separated_list1(newline, nomi64)(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "3");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "1623178306");
    }
}
