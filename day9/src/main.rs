use std::cmp::Ordering;

use itertools::Itertools;

use std::fs;
use std::path::Path;

fn read_file_to_lines_int<P>(filename: P) -> Vec<u64>
where P: AsRef<Path>, {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<u64> = contents.lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    lines
}

fn main() {
    let invalid = part1("./data", 25);
    part2("./data", invalid);
}

fn part1(file: &str, preamble_size: usize) -> u64{
    let mut preamble = read_file_to_lines_int(file);
    let mut values = preamble.split_off(preamble_size);

    for value in values.clone() {
        match is_valid(value, preamble.clone()) {
            true => {
                let migrated_number = values.remove(0);
                preamble.push(migrated_number);
                preamble.remove(0);
                assert_eq!(preamble.len(), preamble_size);
            },
            false  => { println!("Value {} is invalid", value); return value}
        }

    }
    0
}

fn is_valid(value: u64, preamble: Vec<u64>) -> bool {

    match preamble.into_iter().permutations(2).find(|vec| {
        let sum: u64 = vec.iter().sum();
        sum == value
    }) {
        Some(_) => true,
        None    => false
    }
}


fn part2(file: &str, invalid: u64) -> u64 {
    let mut values = read_file_to_lines_int(file);

    let mut result_index: Option<usize> = None;
    while result_index.is_none() {
        result_index = find_sub_slice_matching(&values, &invalid);
        if result_index.is_some() {break};
        values.remove(0);
    }

    let matching_list = &values[..result_index.unwrap()];

    let final_result = (
                matching_list.iter().min().unwrap(),
                matching_list.iter().max().unwrap(),
    );

    println!("Final couple = {:?}", final_result);
    println!("Final result = {}", final_result.0 + final_result.1);
    final_result.0 + final_result.1
}

fn find_sub_slice_matching(slice: &Vec<u64>, goal: &u64) -> Option<usize> {

    // println!("Testing {:?}", slice);

    for i in 0..slice.len() {

        let sum = slice[..i].into_iter().sum::<u64>();
        // println!("    > Sum = {}", sum);
        match sum.cmp(goal) {
            Ordering::Less => (),
            Ordering::Greater => return None,
            Ordering::Equal => return Some(i),
        }
    }
    None
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./datatest", 5), 127);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./datatest", 127), 62);
    }

}
