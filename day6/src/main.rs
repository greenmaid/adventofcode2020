use itertools::Itertools;
use std::fs;
use std::path::Path;

fn _read_file_to_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<String> = contents.lines()
        .map(|s| s.to_string())
        .collect();
    lines
}

fn read_file_and_split_by_blank_line<P>(filename: P, separator: &str) -> Vec<String>
where P: AsRef<Path>, {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let grouped_content: Vec<String> = contents.trim().split("\n\n")
        .map(|s| s.replace("\n", separator))
        .collect();
    grouped_content
}

// main
/////////////////
fn main() {
    let grouped_data = read_file_and_split_by_blank_line("./data", "");
    println!("FINAL part 1 : {}", part1(grouped_data));

    let grouped_data = read_file_and_split_by_blank_line("./data", " ");
    println!("FINAL part 1 : {}", part2(grouped_data));
}
//////////////////


fn part1(grouped_data: Vec<String>) -> usize {

    grouped_data.into_iter().fold(0,|acc, group| {
        group.chars().unique().count().checked_add(acc).unwrap()

    })

    // classic method with 'for'
    // let mut responses = 0;
    // for group in grouped_data {
    //     responses += group.chars().unique().count();
    // }
    // responses
}

fn part2(grouped_data: Vec<String>) -> usize {
    let count = grouped_data.into_iter()
        .fold(0, |accu: usize, group| {
            accu.checked_add(get_common_char(group)).unwrap()
        });
    count
}

fn get_common_char(group: String) -> usize {
    let mut reference_chars = group.split(" ").next().unwrap().to_string();
    for c in reference_chars.clone().chars() {
        match group.split(" ").all(|s| s.contains(c)) {
            true => (),
            false => reference_chars = reference_chars.replace(c, "")
        };
    };
    reference_chars.chars().count()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example_data() {
        let data = read_file_and_split_by_blank_line("./datatest", "");
        assert_eq!(part1(data), 11)
    }

    #[test]
    fn test_part2_with_example_data() {
        let data = read_file_and_split_by_blank_line("./datatest", " ");
        assert_eq!(part2(data), 6)
    }
}
