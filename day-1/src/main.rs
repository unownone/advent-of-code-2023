use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
#[allow(dead_code)]

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug, Clone)]
pub struct NumberMatcher<'a> {
    curr_str: String,
    curr_number: u32,
    possible_numbers: Vec<&'a str>,
}

impl<'a> Default for NumberMatcher<'a> {
    fn default() -> Self {
        Self {
            curr_str: String::from(""),
            curr_number: 10,
            possible_numbers: NUMBERS.to_vec(),
        }
    }
}

impl<'a> NumberMatcher<'a> {
    pub fn add_char(&mut self, c: &char) -> bool {
        if self.curr_number != 10 {
            self.flush_all();
        }
        if self.is_possible_number(c) {
            true
        } else {
            self.flush_all();
            self.curr_str.push(*c);
            false
        }
    }

    fn is_possible_number(&mut self, c: &char) -> bool {
        let maybe_str = self.curr_str.clone() + &c.to_string();
        let possible_numbers: Vec<&str> = self
            .possible_numbers
            .iter()
            .filter(|x| x.starts_with(&maybe_str))
            .map(|x| *x)
            .collect();

        match possible_numbers.len() {
            0 => false,
            1 => {
                if maybe_str.clone() == self.possible_numbers[0] {
                    self.curr_number = parse_number(&maybe_str);
                } else {
                    self.possible_numbers = possible_numbers;
                    self.curr_str.push(*c);
                }
                // self.curr_number = is_string_a_digit(&self.curr_str);
                true
            }
            _ => {
                self.possible_numbers = possible_numbers;
                self.curr_str.push(*c);
                true
            }
        }
    }

    pub fn flush_all(&mut self) {
        self.possible_numbers = NUMBERS.to_vec();
        self.curr_str = String::from("");
        self.curr_number = 10;
    }
}

fn get_digit_from_string(line: &str) -> u32 {
    let mut first_digit: u32 = 10;
    let mut last_digit: u32 = 10;
    let mut get_digit = NumberMatcher::default();

    for c in line.chars() {
        let curr_digit: u32;
        if c.is_digit(10) {
            get_digit.flush_all();
            curr_digit = c.to_digit(10).unwrap();
        } else {
            if get_digit.add_char(&c) {
                if get_digit.curr_number == 10 {
                    continue;
                }
                // println!("FOUND: {:?}", get_digit);
                curr_digit = get_digit.curr_number;
            } else {
                // println!("NOT FOUND: {:?}", get_digit);
                continue;
            }
        }
        if first_digit == 10 {
            first_digit = curr_digit;
        } else {
            last_digit = curr_digit;
        }
    }
    // If last digit is still 10 means it was not set
    if last_digit == 10 {
        println!(
            "first_digit: {} last_digit: {} line: {}",
            first_digit, last_digit, line
        );
        // If the first digit is also 10 means both werent set so both are set to 0
        if first_digit == 10 {
            first_digit = 0;
            last_digit = 0;
        // if first was set means we set first as last ie if string = 7 we do -> 77
        } else {
            last_digit = first_digit;
        }
        println!("Final: {}", (first_digit * 10) + last_digit);
    }
    return (first_digit * 10) + last_digit;
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let lines = BufReader::new(input_file);
    let result: u32 = lines.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let value = get_digit_from_string(&line);
        // println!("{} -> {}", line, value);
        acc + value
    });
    println!("Result: {}", result);
}

fn parse_number(text: &str) -> u32 {
    match text {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 10,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test_part_one() {
        let inpt: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let actual_res = inpt.into_iter().fold(0, |acc, line| {
            let value = get_digit_from_string(&line);
            acc + value
        });
        assert_eq!(actual_res, 142);
    }

    #[test]
    fn run_test_part_two() {
        let inpt: Vec<&str> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let actual_res = inpt.into_iter().fold(0, |acc, line| {
            let value = get_digit_from_string(&line);
            println!("{} -> {}", line, value);
            acc + value
        });
        assert_eq!(actual_res, 281);
    }

    #[test]
    fn run_test_part_three() {
        let inpt: Vec<&str> = vec!["eightwothree", "7pqrstsixteen"];
        let actual_res = inpt.into_iter().fold(0, |acc, line| {
            let value = get_digit_from_string(&line);
            println!("{} -> {}", line, value);
            acc + value
        });
        assert_eq!(actual_res, 159);
    }
}
