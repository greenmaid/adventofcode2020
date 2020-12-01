use std::path::Path;
use itertools::Itertools;
use std::fs;


fn main() {
    part1();
    part2();

}

fn read_file_to_int_vec<P>(filename: P) -> Vec<isize>
where P: AsRef<Path>, {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.lines();
    let values: Vec<isize> = lines.map(|s| s.parse::<isize>().unwrap()).collect();

    values

}

fn part1() {
    let lines = read_file_to_int_vec("./data");
    let couples = lines.into_iter().permutations(2);
    for couple in couples {
        let sum: isize = couple.iter().sum();
        if  sum == 2020 {
            println!("la somme de {:?} vaut 2020", couple);
            let product: isize = couple.iter().product();
            println!("Product = {}", product);
            return ()
        }
    }
}

fn part2() {
    let lines = read_file_to_int_vec("./data");
    let trios = lines.into_iter().permutations(3);
    for trio in trios {
        let sum: isize = trio.iter().sum();
        if  sum == 2020 {
            println!("la somme de {:?} vaut 2020", trio);
            let product: isize = trio.iter().product();
            println!("Product = {}", product);
            return ()
        }
    }
}
