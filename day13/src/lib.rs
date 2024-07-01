use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as nomchar, newline, u8 as nomu8},
    multi::{many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, pairs) = parse_input1(input).unwrap();
    assert!(input.is_empty());
    let total = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| match a.cmp(b) {
            std::cmp::Ordering::Less => Some(i + 1),
            std::cmp::Ordering::Equal => panic!("Packets must not be the same! Error on pair {i}"),
            std::cmp::Ordering::Greater => None,
        })
        .sum::<usize>();
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, mut packets) = parse_input2(input).unwrap();
    assert!(input.is_empty());
    let (_, first_divider) = Packet::parse("[[2]]").unwrap();
    let (_, second_divider) = Packet::parse("[[6]]").unwrap();
    packets.push(first_divider.clone());
    packets.push(second_divider.clone());
    packets.sort();
    let first_index = packets.iter().position(|p| p == &first_divider).unwrap() + 1;
    let second_index = packets.iter().position(|p| p == &second_divider).unwrap() + 1;
    (first_index * second_index).to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    data: Data,
}

impl Packet {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, data) = Data::parse(input)?;
        Ok((input, Self { data }))
    }

    fn parse_pair(input: &str) -> IResult<&str, (Self, Self)> {
        let (input, a) = Self::parse(input)?;
        let (input, _) = newline(input)?;
        let (input, b) = Self::parse(input)?;
        Ok((input, (a, b)))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Data {
    List(Vec<Data>),
    Int(u8),
}

impl Data {
    fn list_from_int(int: u8) -> Self {
        Self::List(vec![Data::Int(int)])
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, data) = delimited(
            nomchar('['),
            separated_list0(
                nomchar(','),
                alt((Self::parse_empty, Self::parse_int, Self::parse)),
            ),
            nomchar(']'),
        )(input)?;
        Ok((input, Self::List(data)))
    }

    fn parse_empty(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("[]")(input)?;
        Ok((input, Self::List(vec![])))
    }

    fn parse_int(input: &str) -> IResult<&str, Self> {
        let (input, int) = nomu8(input)?;
        Ok((input, Self::Int(int)))
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Data::List(s), Data::List(o)) => {
                for (e_s, e_o) in s.iter().zip(o.iter()) {
                    match e_s.cmp(e_o) {
                        std::cmp::Ordering::Equal => continue,
                        cmp => return cmp,
                    }
                }
                s.len().cmp(&o.len())
            }
            (Data::Int(s), Data::Int(o)) => s.cmp(o),
            (s, Data::Int(o)) => s.cmp(&Data::list_from_int(*o)),
            (Data::Int(s), o) => Data::list_from_int(*s).cmp(o),
        }
    }
}

fn parse_input1(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    let (input, lines) = separated_list1(tag("\n\n"), Packet::parse_pair)(input)?;
    Ok((input, lines))
}

fn parse_input2(input: &str) -> IResult<&str, Vec<Packet>> {
    let (input, lines) = separated_list1(many1(newline), Packet::parse)(input)?;
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
        assert_eq!(result, "13");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "140");
    }
}
