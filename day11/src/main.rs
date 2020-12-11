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

type Playground = Vec<Vec<char>>;

fn construct_playground(lines: Vec<String>) -> Playground {
    let width = lines[0].len() + 2;
    let mut playground: Vec<Vec<char>> = vec![ vec!['.'; width] ];
    for line in lines {
        let mut new_elem: Vec<char> = line.trim().chars().collect();
            new_elem.insert(0, '.');
            new_elem.push('.');
        playground.push(new_elem);
    }
    playground.push(vec!['.'; width]);
    playground
}

fn get_playground_str(playground: &Playground) -> String {
    let string = playground.iter()
        .map(|chars| chars.iter().collect::<String>())
        .fold("".to_string(),
            |acc, s| format!("{}\n{}", acc, s));
    string
}

////////////////////////////////////////////////
fn main() {
    let result1 = part1("./data");
    println!("Result part 1 : {}", result1);
    let result2 = part2("./data");
    println!("Result part 2 : {}", result2);
}
////////////////////////////////////////////////////
/////////////////////////////////////////////////////////

fn part1(file: &str) -> usize {
    let lines = _read_file_to_lines(file);
    let mut playground = construct_playground(lines);

    loop {
        let new_playground = process(&playground);
        if are_same_playgrounds(&new_playground, &playground) {
            break
        }
        playground = new_playground;
    }

    count_occupied(&playground)
}

fn count_occupied(play: &Playground) -> usize {
    play.into_iter().flatten().filter(|c| **c == '#').count()
}

fn are_same_playgrounds(play1: &Playground, play2: &Playground) -> bool{
    play1.into_iter().zip(play2.into_iter())
        .map(
            |(vec1, vec2)| {
                vec1.iter().zip(vec2.iter()).all( |(c1,c2)| c1 == c2)
            })
        .all(|x| x == true)
}

fn process(ref_play: &Playground) -> Playground {
    let mut new_playground = ref_play.clone();
    let heigth = ref_play.len();
    let width = ref_play[0].len();

    for y in 1..heigth-1 {
        for x in 1..width-1 {
            match ref_play[y][x] {
                '.' => (),
                _   => {
                    if is_cell_alone(ref_play, y, x) {new_playground[y][x] = '#'};
                    if is_cell_overpopulated(ref_play, y, x) {new_playground[y][x] = 'L'}
                    }
            }
        }
    };
    new_playground
}

fn is_cell_alone(play: &Playground, y: usize, x: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if play[y+i-1][x+j-1] == '#' {return false}
        }
    }
    true
}

fn is_cell_overpopulated(play: &Playground, y: usize, x: usize) -> bool {
    if play[y][x] != '#' {return false}
    let mut counter = 0;
    for i in 0..3 {
        for j in 0..3 {
            if play[y+i-1][x+j-1] == '#' {counter += 1};
            if counter >= 5 {return true}
        }
    };
    false
}


///////////////////////////////
// part2
/////////////////////////////////////////
//////////////////////////////////////////////

fn part2(file: &str) -> usize {
    let lines = _read_file_to_lines(file);
    let mut playground = construct_playground(lines);

    let mut cycles = 0;
    loop {
        cycles += 1;
        if cycles > 100 {panic!("Too long")};
        // println!("Play cycle {}:\n{}", cycles, get_playground_str(&playground));
        let new_playground = process_part2(&playground);
        if are_same_playgrounds(&new_playground, &playground) {
            break
        }
        playground = new_playground;
    }

    count_occupied(&playground)
}

fn process_part2(ref_play: &Playground) -> Playground {
    let mut new_playground = ref_play.clone();
    let heigth = ref_play.len();
    let width = ref_play[0].len();

    for y in 1..heigth-1 {
        for x in 1..width-1 {
            match ref_play[y][x] {
                '.' => (),
                _   => {
                    if is_cell_alone_part2(ref_play, y, x) {new_playground[y][x] = '#'};
                    if is_cell_overpopulated_part2(ref_play, y, x) {new_playground[y][x] = 'L'}
                    }
            }
        }
    };
    new_playground
}


fn count_visible_occupied(play: &Playground, y: usize, x: usize) -> usize {

    let width = play[0].len() as isize;

    let horizontal_right: String = play[y][x+1..].into_iter().copied().collect();
    let horizontal_left: String = play[y][..x].into_iter().copied().rev().collect();

    let vertical: Vec<char> = play.into_iter().map(|vec| vec[x]).collect();
    let vertical_up: String = vertical[..y].iter().copied().rev().collect();
    let vertical_bottom: String = vertical[y+1..].iter().copied().collect();

    let diag_top_bottom: Vec<char> = play.into_iter().enumerate()
        .map(|(i, vec)| {
            let index = (x as isize) - (y as isize) + (i as isize);
            match (index >= 0) && (index < width) {
                true => vec[index as usize],
                false => '?'
            }})
        .collect();
    let top_left: String = diag_top_bottom[..y].iter().copied().rev().collect();
    let bottom_right: String = diag_top_bottom[y+1..].iter().copied().collect();

    let diag_bottom_top: Vec<char> = play.clone().into_iter()
        .enumerate()
        .map(|(i, vec)| {
            let index = (x as isize) + (y as isize) - (i as isize);
            match (index >= 0) && (index < width) {
                true => vec[index as usize],
                false => '?'
            }})
        .collect();
    let top_right: String = diag_bottom_top[..y].iter().copied().rev().collect();
    let bottom_left: String = diag_bottom_top[y+1..].iter().copied().collect();

    // println!("horizontal_left, {:?}", horizontal_left);
    // println!("horizontal_right, {:?}", horizontal_right);
    // println!("vertical_up, {:?}", vertical_up);
    // println!("vertical_bottom, {:?}", vertical_bottom);
    // println!("top_left, {:?}", top_left);
    // println!("bottom_right, {:?}", bottom_right);
    // println!("bottom_left, {:?}", bottom_left);
    // println!("top_right, {:?}", top_right);

    fn is_occupied(str: &str) -> bool {
        let mut string = str.to_string();
        if !(string.contains('#')) {return false};
        string = string.replace(".", "");
        string = string.replace("?", "");
        match string.chars().nth(0) {
            None => false,
            Some('L') => false,
            Some('#') => true,
            Some(c) => panic!("Unexpected char {}", c)
        }
    }

    [
        horizontal_left,
        horizontal_right,
        vertical_up,
        vertical_bottom,
        top_left,
        bottom_right,
        bottom_left,
        top_right,
    ].iter().filter(|s| is_occupied(&s[..])).count()

}


fn is_cell_alone_part2(play: &Playground, y: usize, x: usize) -> bool {
    count_visible_occupied(play, y, x) == 0
}

fn is_cell_overpopulated_part2(play: &Playground, y: usize, x: usize) -> bool {
    if play[y][x] != '#' {return false};
    count_visible_occupied(play, y, x) >= 5
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playground_generation() {
        let lines = _read_file_to_lines("./datatest");
        let playground = construct_playground(lines);
        println!("Playground:\n{}", get_playground_str(&playground));
        // assert_eq!(0,1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("./datatest"), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./datatest"), 26);
    }

    #[test]
    fn test_partial_part2() {
        let lines = _read_file_to_lines("./datatest3");
        let playground = construct_playground(lines);
        let count = count_visible_occupied(&playground,1,4);
        println!("Count => {}", count);
        println!("");
        let count = count_visible_occupied(&playground,4,1);
        println!("Count => {}", count);
        println!("");
        let count = count_visible_occupied(&playground,8,3);
        println!("Count => {}", count);
        // assert_eq!(0,1);
    }

    #[test]
    fn test_part2_extract_direction() {
        let lines = _read_file_to_lines("./datatest2");
        let playground = construct_playground(lines);
        println!("Playground:\n{}", get_playground_str(&playground));
        assert_eq!(count_visible_occupied(&playground,4,4), 0);
        assert_eq!(count_visible_occupied(&playground,1,3), 5);

        let lines = _read_file_to_lines("./datatest3");
        let playground = construct_playground(lines);
        println!("Playground:\n{}", get_playground_str(&playground));
        assert_eq!(count_visible_occupied(&playground,1,4), 0);
        assert_eq!(count_visible_occupied(&playground,1,3), 1);
        assert_eq!(count_visible_occupied(&playground,6,9), 2);
    }
}
