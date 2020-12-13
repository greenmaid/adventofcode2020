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

fn parse_navigation(lines: Vec<String>) -> Vec<(String, isize)> {
    lines.into_iter()
        .map(|s| (s[..1].to_string(), s[1..].parse::<isize>().unwrap()))
        .collect()
}

fn main() {
    let result1 =part1("./data");
    println!("Result part1 = {}", result1);
    let result2 =part2("./data");
    println!("Result part2 = {}", result2);
}

#[derive (PartialEq, Clone, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

// Ship for part 1
#[derive (Debug)]
struct Ship1 {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Ship1 {
    fn execute_cmd(&mut self, instruction: (String, isize)) {
        let (cmd, value) = instruction;
        match &cmd[..] {
            "N" => (self.y += value),
            "S" => (self.y -= value),
            "E" => (self.x += value),
            "W" => (self.x -= value),
            "L" => (self.turn("-", value)),
            "R" => (self.turn("+", value)),
            "F" => (self.move_forward(value)),
            other => panic!("Unknown command {}", other)
        }
    }

    fn turn(&mut self, rotation: &str, degrees: isize) {
        let directions = vec![
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
            ];

        let rotation_steps = (degrees / 90) as usize;

        // { print!("{:?}", directions); print!(" {} ", current); println!(" {} ", rotation_steps)};

        self.direction = match rotation {
            "+" => {
                let current = directions.iter().position(|dir| *dir == self.direction).unwrap();
                directions.into_iter().nth((current+rotation_steps)%4).unwrap()
            }
            "-" => {
                let current = directions.iter().rev().position(|dir| *dir == self.direction).unwrap();
                directions.into_iter().rev().nth((current+rotation_steps)%4).unwrap()
            }
            _   => panic!("Rotation should be + or -")
        };
    }

    fn move_forward(&mut self, value: isize) {
        match self.direction {
            Direction::NORTH => self.y += value,
            Direction::EAST => self.x += value,
            Direction::SOUTH => self.y -= value,
            Direction::WEST => self.x -= value,
        }
    }
}

fn part1(lines: &str) -> isize {
    let navigation = parse_navigation(read_file_to_lines(lines));
    let mut ferry = Ship1 {x: 0, y: 0, direction: Direction::EAST,};
    for instruction in navigation {
        // println!("Next move {:?}", instruction);
        ferry.execute_cmd(instruction);
        // println!("Ferry : {:#?}", ferry);
    }

    ferry.x.abs() + ferry.y.abs()
}


/////////////////////////////////////////////////
// part2
/////////////////////////////////


// Ship for part 2
#[derive (Debug)]
struct Ship2 {
    coords: Coordinates,
    waypoint: Coordinates,
}

#[derive (Debug)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Ship2 {
    fn execute_cmd(&mut self, instruction: (String, isize)) {
        let (cmd, value) = instruction;
        match &cmd[..] {
            "N" => (self.waypoint.y += value),
            "S" => (self.waypoint.y -= value),
            "E" => (self.waypoint.x += value),
            "W" => (self.waypoint.x -= value),
            "L" => (self.turn("-", value)),
            "R" => (self.turn("+", value)),
            "F" => (self.move_forward(value)),
            other => panic!("Unknown command {}", other)
        }
    }

    fn turn(&mut self, rotation: &str, degrees: isize) {

        let rotation_steps = match rotation {
            "+" => (degrees / 90) % 4,  // pi/2 equivalent
            "-" => (4 - (degrees / 90)) % 4,
            _ => panic!("Rotation should be + or -"),
        };

        // // println!("rotations : {} => {}", rotation, rotation_steps);

        // assert!(rotation_steps <= 4 );
        // assert!(rotation_steps >= 0 );


        for _ in 0..rotation_steps {
            match self.waypoint.x >= 0 {
                true  => match self.waypoint.y >= 0 {
                    true => {self.waypoint.x = self.waypoint.x * -1; let tmp = self.waypoint.x ; self.waypoint.x = self.waypoint.y; self.waypoint.y = tmp; },
                    false => {self.waypoint.x = self.waypoint.x * -1; let tmp = self.waypoint.x ; self.waypoint.x = self.waypoint.y; self.waypoint.y = tmp; },
                }
                false => match self.waypoint.y >= 0 {
                    true => {self.waypoint.x = self.waypoint.x * -1; let tmp = self.waypoint.x ; self.waypoint.x = self.waypoint.y; self.waypoint.y = tmp; },
                    false => { self.waypoint.x = self.waypoint.x * -1; let tmp = self.waypoint.x ; self.waypoint.x = self.waypoint.y; self.waypoint.y = tmp; },
                }
            }
        }
    }

    fn move_forward(&mut self, value: isize) {
        self.coords.x += self.waypoint.x * value;
        self.coords.y += self.waypoint.y * value;
    }
}

fn part2(lines: &str) -> isize {
    let navigation = parse_navigation(read_file_to_lines(lines));
    let mut ferry = Ship2 {coords: Coordinates {x: 0, y: 0}, waypoint: Coordinates {x: 10, y: 1}};
    for instruction in navigation {
        // println!("Next move {:?}", instruction);
        ferry.execute_cmd(instruction);
        // println!("Ferry : {:#?}", ferry);
    }

    ferry.coords.x.abs() + ferry.coords.y.abs()
}










mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./datatest"), 25);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1("./datatest2"), 0);
    }

    #[test]
    fn test_turn() {
        let mut ship = Ship1 {x: 0, y: 0, direction: Direction::EAST,};
        ship.turn("+", 90);
        assert_eq!(ship.direction, Direction::SOUTH);
        ship.turn("+", 90);
        ship.turn("+", 90);
        assert_eq!(ship.direction, Direction::NORTH);
        ship.turn("-", 180);
        assert_eq!(ship.direction, Direction::SOUTH);
        ship.turn("-", 90);
        assert_eq!(ship.direction, Direction::EAST);
        ship.turn("-", 180);
        assert_eq!(ship.direction, Direction::WEST);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./datatest"), 286);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2("./datatest2"), 0);
    }
}
