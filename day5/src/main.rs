use std::fs;
use std::path::Path;

fn read_file_to_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<String> = contents.lines()
        .map(|s| s.to_string())
        .collect();
    lines
}

fn main() {
    let lines = read_file_to_lines("./data");
    let seats = scan_lines_part1(lines);
    println!("");
    part2(seats);
}

fn part2(seats: Vec<u64>) {
    // Iteration range boundaries should be calculated from seats min and max... but I'm lazy
    for i in 59u64..905 {
        if !(seats.contains(&i)) {
            println!("Missing (Result part 2): {}", i)
        }

    }

}

fn scan_lines_part1(lines: Vec<String>) -> Vec<u64> {
    let seats = lines.into_iter().map(|s| scan_code(s));
    let max = seats.clone().max().unwrap();
    let min = seats.clone().min().unwrap();
    println!("Max (Result part 1) : {}", max);
    println!("Min : {}", min);
    seats.collect::<Vec<u64>>()
}

fn scan_code(code: String) -> u64 {
    let row = get_row_from_code(&code[0..7]);
    let column = get_row_from_code(&code[7..10]);
    row * 8 + column
}

fn get_row_from_code(code: &str) -> u64 {

    let row_bin: String = code.chars()
                .map(|c| match c{
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => panic!("Should not happen !!")
                })
                .collect();
    u64::from_str_radix(&row_bin, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_row() {
        assert_eq!(get_row_from_code("FFFF"), 0);
        assert_eq!(get_row_from_code("FBFBBFF"), 44);
        assert_eq!(get_row_from_code("RLR"), 5);
        assert_eq!(get_row_from_code("BFFFBBF"), 70);
        assert_eq!(get_row_from_code("RRR"), 7);
    }

    #[test]
    fn test_id() {
        assert_eq!(scan_code("BFFFBBFRRR".to_string()), 567);
        assert_eq!(scan_code("FFFBBBFRRR".to_string()), 119);
        assert_eq!(scan_code("BBFFBBFRLL".to_string()), 820);
    }
}
