use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, snafus) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let snafu_sum = snafus.into_iter().sum::<Snafu>();
    format!("{}", snafu_sum)
}

pub fn process_part2(_input: &str) -> String {
    // there is no part 2 the last day!
    "".to_string()
}

#[derive(Debug, Default)]
struct Snafu {
    value: isize,
}

impl Snafu {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, chars) = many1(one_of("=-012"))(input)?;
        let value = chars
            .into_iter()
            .rev()
            .enumerate()
            .fold(0_isize, |acc, (pow, c)| {
                let digit: isize = match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    x => unreachable!("Invalid character for Snafu: {x}"),
                };
                acc + digit * 5_isize.pow(pow as u32)
            });
        assert!(value > 0);
        let value = value.abs();
        Ok((input, Self { value }))
    }
}

impl std::ops::Add<Self> for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl std::iter::Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, el| acc + el)
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn digit_form_of(n: isize) -> char {
            match n {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                x => unreachable!("Invalid Snafu digit: {x}"),
            }
        }
        let mut value = self.value;
        if (0..3).contains(&value) {
            return write!(f, "{}", value);
        }
        let five = 5_isize;
        let mut result = String::new();
        let mut power_of_5 = (self.value as f64).log(5.0).ceil() as u32;
        if value <= five.pow(power_of_5) / 2 {
            power_of_5 -= 1;
        }
        loop {
            let digit_value = five.pow(power_of_5);
            let boundary = digit_value / 2;
            let mut digit = -2;
            while (value - (digit * digit_value)) > boundary {
                digit += 1;
            }
            result.push(digit_form_of(digit));
            value = (value - digit * digit_value) % digit_value;
            if power_of_5 == 0 {
                break;
            }
            power_of_5 -= 1;
        }
        f.write_str(&result)
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Snafu>> {
    let (input, lines) = separated_list1(newline, Snafu::parse)(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn translate_into_snafu() {
        let pairs = [
            (1_isize, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
            (1747, "1=-0-2"),
            (906, "12111"),
            (198, "2=0="),
            (11, "21"),
            (201, "2=01"),
            (31, "111"),
            (1257, "20012"),
            (32, "112"),
            (353, "1=-1="),
            (107, "1-12"),
            (7, "12"),
            (3, "1="),
            (37, "122"),
        ];
        pairs
            .into_iter()
            .for_each(|(n, s)| assert_eq!(Snafu { value: n }.to_string(), s))
    }

    #[test]
    fn more_snafu_tests() {
        let tests = ["1=000="];
        tests
            .into_iter()
            .for_each(|t| assert_eq!(format!("{}", Snafu::parse(t).unwrap().1), t));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "2=-1=0");
    }
}
