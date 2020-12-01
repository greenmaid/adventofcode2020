use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() {

    if let Ok(lines1) = read_lines("./data") {
        for line1 in lines1 {
            if let Ok(number1) = line1 {
                if let Ok(lines2) = read_lines("./data") {
                    for line2 in lines2 {
                        if let Ok(number2) = line2 {
                            let int1: i32 = number1.parse().unwrap();
                            let int2: i32 = number2.parse().unwrap();
                            if  int1 + int2 == 2020 {
                                println!("{} + {} = 2020", number1, number2);
                                println!("{} * {} = {}", number1, number2, int1 * int2);
                                return ()
                            }
                        }
                    }
                }

            }
        }
    }

}

fn part2() {

    if let Ok(lines1) = read_lines("./data") {
        for line1 in lines1 {
            if let Ok(number1) = line1 {
                if let Ok(lines2) = read_lines("./data") {
                    for line2 in lines2 {
                        if let Ok(number2) = line2 {

                            if let Ok(lines3) = read_lines("./data") {
                                for line3 in lines3 {
                                    if let Ok(number3) = line3 {


                                        let int1: i32 = number1.parse().unwrap();
                                        let int2: i32 = number2.parse().unwrap();
                                        let int3: i32 = number3.parse().unwrap();
                                        if  int1 + int2 + int3 == 2020 {
                                            println!("{} + {} + {} = 2020", number1, number2, number3);
                                            println!("{} * {} * {} = {}", number1, number2, number3, int1 * int2 * int3);
                                            return ()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

            }
        }
    }

}
