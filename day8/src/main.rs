// use std::collections::{HashMap, hash_map::RandomState};
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
    // part1()
    part2()
}



fn part1() {
    let lines = read_file_to_lines("./data");
    let cmd = create_command_list(lines);
    let (result, _) = play(cmd);
    println!("Result: {}", result);
}



fn part2() {
    let mut iteration = 0;
    while true {
        let lines = read_file_to_lines("./data");
        let cmd = get_cmd_list_with_nth_item_mutated(lines, iteration);
        let (result, is_terminated) = play(cmd);

        println!("Iteration {} -> Finish: {}", iteration, is_terminated);
        iteration += 1;

        if is_terminated {
            println!("Final result: {}", result);
            break
        };
    }



}


fn get_cmd_list_with_nth_item_mutated(lines: Vec<String>, n: usize) -> Vec<Instruction> {

    let ref_lines = lines.clone();
    let ref_cmds = create_command_list(lines);
    // index du la commande à changer
    let (index , instruction_to_change) = ref_cmds.iter().enumerate()
        .filter(|(_i,instruc)| match instruc.command {
                Command::Jmp | Command::Nop => true,
                _ => false
        })
        .nth(n).unwrap();

    let new_command = match instruction_to_change.command {
        Command::Jmp => Command::Nop,
        Command::Nop => Command::Jmp,
        _ => panic!("Should not happend !!!")
    };

    let mut cmds = create_command_list(ref_lines);
    cmds[index].command = new_command;

    cmds

}



enum Command {
    Nop,
    Acc,
    Jmp,
    Fin,
}

struct Instruction {
    command: Command,
    value: i32,
}

fn parse_instruction(instruction_line: &str) -> Instruction {
    let mut iterator = instruction_line.split(" ");
    let cmd = iterator.next().unwrap();
    let value: i32 = iterator.next().unwrap().parse().unwrap();

    let command = match cmd {
        "nop" => Command::Nop,
        "acc" => Command::Acc,
        "jmp" => Command::Jmp,
        "fin" => Command::Fin,
        _ => panic!("unexpected value {}", cmd)
    };

    Instruction {command, value}
}

fn create_command_list(lines: Vec<String>) -> Vec<Instruction> {
    let mut cmd_list = vec![];
    for line in lines {
        cmd_list.push(parse_instruction(&line));
    };
    cmd_list
}

#[derive(Debug)]
struct State {
    index: i32,
    accumulator: i32,
    executed_indices: Vec<i32>,
    terminated: bool,
}

fn play(cmd_list: Vec<Instruction>) -> (i32, bool) {

    let mut state = State {
        index: 0,
        accumulator: 0,
        executed_indices: vec![0],
        terminated: false,
    };

    while true {
        let current_command = cmd_list.iter().nth(state.index as usize).unwrap();
        state = execute_instruction(current_command, state);
        // println!("State: {:#?}", state);
        match &state.executed_indices.contains(&state.index) {
            true => break,
            false => (),
        }
        state.executed_indices.push(state.index);
    };

    // println!("State: {:#?}", state);
    (state.accumulator, state.terminated)
}

fn execute_instruction(instruction: &Instruction, mut state: State) -> State {
    let _result = match &instruction.command {
        Command::Nop => {state.index +=1;},
        Command::Acc => {state.accumulator += instruction.value; state.index +=1;},
        Command::Jmp => {state.index += instruction.value;},
        Command::Fin => {state.terminated = true;},
    };
    state
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_file_to_lines("./datatest");
        let cmd = create_command_list(lines);
        assert_eq!(play(cmd), (5, false));
    }
}
