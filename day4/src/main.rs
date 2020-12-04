use regex::{Regex};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let lines = read_file_to_lines("./data");
    part1(lines);
    let lines = read_file_to_lines("./data");
    part2(lines);

}

fn part1(lines: Vec<String>) -> i32 {
    let mut counter = 0;
    for data in isolate_passport_data(lines) {
        if is_valid_part1(parse_passport(data)) {
            counter += 1;
        }
    }
    println!("Result part 1 = {}", counter);
    counter
}

fn part2(lines: Vec<String>) -> i32 {
    let mut counter = 0;
    for data in isolate_passport_data(lines) {
        if is_valid_part2(parse_passport(data)) {
            counter += 1;
        }
    }
    println!("Result part 2 = {}", counter);
    counter
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

fn isolate_passport_data(lines: Vec<String>) -> Vec<String> {
    let mut isolated_data: Vec<String> = Vec::new();
    let mut buffer = "".to_string();
    for line in lines {
        let data = line.trim();
        if data == "" {
            isolated_data.push(buffer.trim().to_string());
            buffer = "".to_string();
        } else {
            buffer = [&buffer, data].join(" ");
        }
    }
    isolated_data
}

fn parse_passport(data: String) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    for input in data.split(" ") {
        let mut iterator = input.split(":");
        let key = iterator.next().unwrap().to_string();
        let value = iterator.next().unwrap().to_string();
        hashmap.insert(key, value);
    };
    hashmap
}

fn is_valid_part1(data: HashMap<String, String>) -> bool {
    if let None = data.get("byr") {return false};
    if let None = data.get("iyr") {return false};
    if let None = data.get("eyr") {return false};
    if let None = data.get("hgt") {return false};
    if let None = data.get("hcl") {return false};
    if let None = data.get("ecl") {return false};
    if let None = data.get("pid") {return false};
    true
}

fn is_valid_part2(data: HashMap<String, String>) -> bool {

    // for item in data.iter() {
    //     println!("{:?}", item);
    // }
    let regex_year = Regex::new(r"^\d{4}$").unwrap();
    match data.get("byr") {
        None => return false,
        Some(str) => {
            match regex_year.captures(str) {
                None => return false,
                Some(year) => {
                    let int: i32 = year[0].parse().unwrap();
                    if !(is_valid_value(int, 1920,2002)) {
                        return false
                    }
                }
            }
        }
    }

    match data.get("iyr") {
        None => return false,
        Some(str) => {
            match regex_year.captures(str) {
                None => return false,
                Some(year) => {
                    let int: i32 = year[0].parse().unwrap();
                    if !(is_valid_value(int, 2010,2020)) {
                        return false
                    }
                }
            }
        }
    }

    match data.get("eyr") {
        None => return false,
        Some(str) => {
            match regex_year.captures(str) {
                None => return false,
                Some(year) => {
                    let int: i32 = year[0].parse().unwrap();
                    if !(is_valid_value(int, 2020,2030)) {
                        return false
                    }
                }
            }
        }
    }

    let regex_height = Regex::new(r#"^(?P<value>\d+)(?P<unit>.{2})$"#).unwrap();
    match data.get("hgt") {
        None => return false,
        Some(str) => {
            match regex_height.captures(str) {
                None => return false,
                Some(caps) => {
                    let value: i32 = caps["value"].parse().unwrap();
                    let unit = &caps["unit"];
                    match unit {
                        "cm" => {
                            if !(is_valid_value(value, 150,193)) {
                                return false
                        }},
                        "in" => {
                            if !(is_valid_value(value, 59,76)) {
                                return false
                        }},
                        _ => return false
                    }
                }
            }
        }
    }

    let regex_hair_color = Regex::new(r#"^#[a-f0-9]{6}$"#).unwrap();
    match data.get("hcl") {
        None => return false,
        Some(str) => {
            match regex_hair_color.captures(str) {
                None => return false,
                Some(_) => ()
            }
        }
    }

    match data.get("ecl") {
        None => return false,
        Some(str) => {
            match &str[..] {
                "amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth" => (),
                _ => return false
            }
        }
    }

    let regex_pid = Regex::new(r#"^\d{9}$"#).unwrap();
    match data.get("pid") {
        None => return false,
        Some(str) => {
            match regex_pid.captures(str) {
                None => return false,
                Some(_) => ()
            }
        }
    }

    true
}

fn is_valid_value(value: i32, min: i32, max: i32) -> bool {
    if (value < min ) || (value > max) {
        return false
    }
    return true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_example_data() {
        let lines = read_file_to_lines("./datatest");
        let mut iterator = isolate_passport_data(lines).into_iter();
        let expected1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".to_string();
        assert_eq!(iterator.next(), Some(expected1));
        let expected2 = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string();
        assert_eq!(iterator.next(), Some(expected2));

    }

    #[test]
    fn test_hashmap() {
        let mut hashmap = HashMap::new();
        hashmap.insert("key", "value");
        assert_eq!(hashmap["key"], "value");
        assert_eq!(hashmap.get("other"), None);
    }

    #[test]
    fn test_part2() {
        let lines = read_file_to_lines("./datatest2_invalid");
        assert_eq!(part2(lines), 0);
        let lines = read_file_to_lines("./datatest2_valid");
        assert_eq!(part2(lines), 4);
    }

}
