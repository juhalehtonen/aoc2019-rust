pub fn solve(input : String) {
    let result_part1 = solve_part1(input);
    println!("Result for day 2 part 1 is {:?}", result_part1);
}

#[derive(Debug)]
struct OpcodeComputer {
    opcodes: Vec<u32>
}

#[derive(Debug)]
#[derive(PartialEq)]
enum OpcodeInstruction {
    Add,
    Multiply,
    Halt,
    NoOp
}

fn solve_part1(input : String) -> u32 {
    // 1. Turn string into vector of u32s
    let mut instructions: Vec<u32> = input.split_terminator(',').map(|x| x.parse::<u32>().unwrap()).collect();
    // 2. Restore status to 1202 program alarm
    instructions = restore_1202_program_alarm(instructions);
    println!("{:?}", &instructions);

    // 3. Build an initial mutable struct to represent our state
    let mut computer = OpcodeComputer {
        opcodes: instructions
    };

    println!("Computer before mutations: {:?}", computer);

    // 3. Loop through our computer opcodes and add 4 to the pointer on every iteration, allowing us
    // to jump to relative points in the vector.
    let mut instruction_pointer = 0;
    while instruction_pointer < computer.opcodes.len() {
        let opcode = computer.opcodes[instruction_pointer];
        let opcode_action = handle_opcode(opcode);

        match opcode_action {
            OpcodeInstruction::Add | OpcodeInstruction::Multiply => {
                let first_action_index = computer.opcodes[instruction_pointer + 1] as usize;
                let second_action_index = computer.opcodes[instruction_pointer + 2] as usize;
                let result_index = computer.opcodes[instruction_pointer + 3] as usize;

                match opcode_action {
                    OpcodeInstruction::Add => {
                        computer.opcodes[result_index] = computer.opcodes[first_action_index] + computer.opcodes[second_action_index];
                    }
                    OpcodeInstruction::Multiply => {
                        computer.opcodes[result_index] = computer.opcodes[first_action_index] * computer.opcodes[second_action_index];
                    }
                    _ => println!("Invalid opcode: {:?}", opcode_action)
                }
            },
            OpcodeInstruction::Halt => {
                println!("Computer after mutations: {:?}", computer);
                return computer.opcodes[0]
            },
            _ => println!("Invalid opcode: {:?}", opcode_action)
        }

        // Move to the next group of 4 pointers
        instruction_pointer += 4;
    }

    return computer.opcodes[0]
}

/**
 * Run at the very beginning to restore the 1202 program alarm to the state.
 * Replace position 1 with the value 12 and replace position 2 with the value 2.
 */
fn restore_1202_program_alarm(instructions: Vec<u32>) -> Vec<u32> {
    let mut insts = instructions;
    insts[1] = 12;
    insts[2] = 2;
    return insts;
}

/**
 * Match opcode to a specific operation.
 */
fn handle_opcode(opcode : u32) -> OpcodeInstruction {
    match opcode {
        1 => OpcodeInstruction::Add,
        2 => OpcodeInstruction::Multiply,
        99 => OpcodeInstruction::Halt,
        _ => OpcodeInstruction::NoOp,
    }
}