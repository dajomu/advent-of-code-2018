use std::fs;

fn main() {
    let instructions = fs::read_to_string("instructions.txt")
        .expect("Something went wrong reading the file");

    let instruction_set:Vec<i32> = instructions.split(",").map(|instruction_string| instruction_string.parse::<i32>().unwrap()).collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let intcode_result = intcode_computer(instruction_set.clone(),  vec![(1, noun), (2, verb)]);
            if intcode_result == 19690720 {
                println!("Success, noun and verb: {}, {}", noun, verb);
            }
        }
    }
}

fn intcode_computer(instruction_set:Vec<i32>, start_settings:Vec<(usize, i32)>) -> i32 {
    let altered_instructions:Vec<i32> = apply_initial_changes(instruction_set, start_settings);
    let processed_instructions = recurse_instruction(altered_instructions, 0);
    return processed_instructions[0];
}

fn recurse_instruction (instruction_set:Vec<i32>, current_instruction:usize) -> Vec<i32> {
    let mut new_instructions:Vec<i32> = instruction_set.clone();
    if instruction_set[current_instruction] == 1 {
        new_instructions = recurse_instruction(operation1(instruction_set, current_instruction), current_instruction + 4);
    } else if instruction_set[current_instruction] == 2 {
        new_instructions = recurse_instruction(operation2(instruction_set, current_instruction), current_instruction + 4);
    } else if instruction_set[current_instruction] == 99 {
    } else {
        println!("no op at: {}", current_instruction);
        new_instructions = recurse_instruction(instruction_set, current_instruction + 4);
    }
    return new_instructions
}

fn apply_initial_changes (instruction_set:Vec<i32>, changes:Vec<(usize, i32)>) -> Vec<i32> {
    let mut new_instructions:Vec<i32> = instruction_set.clone();
    for change in changes {
        new_instructions[change.0] = change.1;
    }
    return new_instructions
}

fn operation1 (instruction_set:Vec<i32>, current_instruction:usize) -> Vec<i32> {
    // add two numbers after current instruction and save them in place of third instruction number
    let mut new_instructions:Vec<i32> = instruction_set.clone();
    new_instructions[instruction_set[current_instruction + 3] as usize] = instruction_set[instruction_set[current_instruction + 1] as usize] + instruction_set[instruction_set[current_instruction + 2] as usize];
    return new_instructions
}

fn operation2 (instruction_set:Vec<i32>, current_instruction:usize) -> Vec<i32> {
    // multiply two numbers after current instruction and save them in place of third instruction number
    let mut new_instructions:Vec<i32> = instruction_set.clone();
    new_instructions[instruction_set[current_instruction + 3] as usize] = instruction_set[instruction_set[current_instruction + 1] as usize] * instruction_set[instruction_set[current_instruction + 2] as usize];
    return new_instructions
}
