use std::path::Path;
use std::fs;
use regex::{Regex, Captures};

fn main() {
    let lines = read_file_to_lines("./data");
    part1(lines);
    let lines = read_file_to_lines("./data");
    part2(lines);

}

fn read_file_to_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<String> = contents.lines()
        .map(|s| s.to_string())
        .collect();

    lines

}

fn parse_line(line: &str) -> Captures {
    let re = Regex::new(r"
        ^
        (?P<min>\d+)
        -
        (?P<max>\d+)
        \s
        (?P<char>.?)
        :\s
        (?P<value>.+)
        $
        ").unwrap();
    re.captures(line).unwrap()
}

fn part1(lines: Vec<String>) -> usize {

    let mut counter: usize = 0;
    for line in lines {

        let caps = parse_line(&line);
        let matches  = caps["value"].matches(&caps["char"]).count();
        let min: usize = caps["min"].parse().unwrap();
        let max: usize = caps["max"].parse().unwrap();

        if matches >= min && matches <= max {
            // println!("Success");
            counter = counter + 1;
        } else {
            // println!("Failure");
        }

    }
    println!("Result part1: {}", counter);
    counter
}

fn part2(lines: Vec<String>) -> usize {

    let mut counter: usize = 0;
    for line in lines {

        let caps = parse_line(&line);
        let value  = &caps["value"];
        let c  = &caps["char"].chars().next().unwrap();
        let position1: usize = caps["min"].parse().unwrap();
        let position2: usize = caps["max"].parse().unwrap();

        if (&value.chars().nth(position1 - 1).unwrap() == c) ^ (&value.chars().nth(position2 - 1).unwrap() == c) {
            // println!("Success");
            counter = counter + 1;
        } else {
            // println!("Failure");
        }

    }
    println!("Result part2: {}", counter);
    counter
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let test_line = "1-3 g: gsng";
        let caps = parse_line(test_line);
        assert_eq!(&caps["min"], "1");
        assert_eq!(&caps["max"], "3");
        assert_eq!(&caps["char"], "g");
        assert_eq!(&caps["value"], "gsng");

        let test_line = "12-16 f: ffqlfhzflqffffkfz";
        let caps = parse_line(test_line);
        assert_eq!(&caps["min"], "12");
        assert_eq!(&caps["max"], "16");
        assert_eq!(&caps["char"], "f");
        assert_eq!(&caps["value"], "ffqlfhzflqffffkfz");
    }

    #[test]
    fn test_xor_op() {
        assert_eq!(true ^ false, true);
        assert_eq!(true ^ true, false);
        assert_eq!(5u8 ^ 1u8, 4);
        assert_eq!(5u8 ^ 2u8, 7);
    }

    #[test]
    fn test_part2() {
        let lines = read_file_to_lines("./datatest");
        assert_eq!(part2(lines), 1);
    }
}
