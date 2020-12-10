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
    let result1 = part1("./data");
    println!("Result part1 : {}", result1);
    part2("./data");
}

fn part1(file: &str) -> u64 {

    let mut values = read_file_to_lines_int(file);
        values.push(0);
        values.sort();


    let diffs: Vec<u64> = values.clone().into_iter().zip(values.into_iter().skip(1)).map(
        |(a,b )| b - a
    ).collect();

    println!("Diffs = {:?}", diffs);

    let diff1 = diffs.iter().filter(|x| **x == 1 ).count();
    let diff3 = diffs.iter().filter(|x| **x == 3 ).count();

    println!("Diff1 = {} / Diff3 = {}", diff1, diff3);
    (diff1 * (diff3+1)) as u64
}

fn part2(file: &str) -> u64 {

    let mut values = read_file_to_lines_int(file);
        values.push(0);
        values.sort();


    let diffs: Vec<u64> = values.clone().into_iter().zip(values.into_iter().skip(1)).map(
        |(a,b )| b - a
    ).collect();
    println!("diffs = {:?}", diffs);


    let mut permuts = 1;
    let mut i = 0;

    while i < diffs.len() {
        match start_of_series(i, &diffs) {
            None => {i += 1;println!("{} : nothing - permuts = {}", i, permuts);},
            Some(113) => {i += 2; permuts = permuts * 2;println!("{} : found 113 - permuts = {}", i, permuts);},
            Some(1113) => {i += 3; permuts = permuts * 4;println!("{} : found 1113 - permuts = {}", i, permuts);},
            Some(11113) => {i += 4; permuts = permuts * 7;println!("{} : found 11113 - permuts = {}", i, permuts);},
            Some(e)    => panic!("return {} ???? ", e),
        }
    }

    println!("Result : {}", permuts);

    permuts
}

fn start_of_series(index: usize, diffs: &Vec<u64>) -> Option<i32> {

    match diffs[index] {
        3 => {return None},
        1 => match diffs.iter().nth(index+1) {
            None | Some(3) => {return None},
            Some(1) => match diffs.iter().nth(index+2) {
                None | Some(3) => return Some(113),
                Some(1) => match diffs.iter().nth(index+3) {
                    None | Some(3) => return Some(1113),
                    Some(1) => match diffs.iter().nth(index+4) {
                        None | Some(3) => return Some(11113),
                        Some(1) => panic!("Not expected !!"),
                        _ => panic!("Should get 1 or 3")
                        },
                    _ => panic!("Should get 1 or 3")
                    },
                _ => panic!("Should get 1 or 3")
                },
            _ => panic!("Should get 1 or 3")
            },
        _ => panic!("Should get 1 or 3")
        };

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./datatest"), 220);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./datatest2"), 8);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2("./datatest"), 19208);
    }

    // #[test]
    // fn test_nth() {
    //     let vec = vec![0,1,2,3,4];

    //     let next = &vec[0];
    //     println!("{}", next);

    //     let next = &vec.iter().nth(1);
    //     println!("{:?}", next);

    //     let next = &vec.iter().nth(2);
    //     println!("{:?}", next);

    //     assert_eq!(0,1)

    // }

}
