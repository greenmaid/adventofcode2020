use std::path::Path;
use std::fs;

fn main() {
    let lines = read_file_to_lines("./data");
    part1(lines);
    println!("");
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

fn part1(lines: Vec<String>) {
    scan(lines, 3, 1);
}

fn part2(lines: Vec<String>) {
    let lines_cloned = lines.clone();
    let scenario1 = scan(lines_cloned, 1, 1);
    let lines_cloned = lines.clone();
    let scenario2 = scan(lines_cloned, 3, 1);
    let lines_cloned = lines.clone();
    let scenario3 = scan(lines_cloned, 5, 1);
    let lines_cloned = lines.clone();
    let scenario4 = scan(lines_cloned, 7, 1);
    let lines_cloned = lines.clone();
    let scenario5 = scan(lines_cloned, 1, 2);

    println!("Product = {}", scenario1 * scenario2 * scenario3 * scenario4 * scenario5);

}

fn scan(mut lines: Vec<String>, right_decal: usize, down_decal: usize) -> usize {

    let mut hash_counter: usize = 0;
    let mut position: usize = 0;

    for _ in 0..down_decal { lines.remove(0); }

    for line in lines.into_iter().step_by(down_decal) {

        position = (position + right_decal) % line.len();

        if is_hash(line, position) {
            hash_counter += 1;
            // println!("Hash found at position {}", position)
        }

    }
    println!("Result part scan right{}, down{}: {}", right_decal, down_decal, hash_counter);
    hash_counter
}

fn is_hash(line: String, position: usize) -> bool {
    let character: char = line.chars().nth(position).unwrap();
    if character == '#' {
        return true
    }
    false
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_example_data() {
        let lines = read_file_to_lines("./datatest");
        assert_eq!(scan(lines,3,1), 7);
        let lines = read_file_to_lines("./datatest");
        assert_eq!(scan(lines,1,1), 2);
        let lines = read_file_to_lines("./datatest");
        assert_eq!(scan(lines,5,1), 3);
        let lines = read_file_to_lines("./datatest");
        assert_eq!(scan(lines,7,1), 4);
        let lines = read_file_to_lines("./datatest");
        assert_eq!(scan(lines,1,2), 2);
    }

    #[test]
    fn test_iter_step() {
        let vector = vec![1, 2, 3, 4, 5, 6];
        let mut iterator = vector.iter().step_by(2);
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), None);

    }

}
