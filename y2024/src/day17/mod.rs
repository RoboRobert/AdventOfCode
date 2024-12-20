#[derive(Debug,Clone)]
struct Registers {
    A: i128,
    B: i128,
    C: i128,
}

// This converts a combo operand to a value
fn get_combo(registers: Registers, operand: i128) -> i128 {
    let mut ret_value = 0;
    match operand {
        7 => println!("invalid combo operand!"),
        4 => return registers.A,
        5 => return registers.B,
        6 => return registers.C,
        _ => return operand,
    }

    return operand;
}

fn do_program(registers: &mut Registers, program: &mut Vec<i128>) -> String {
    let mut ret_string = String::new();
    let mut i: usize = 0;
    // Loop until the program breaks
    while (i < program.len()) {
        let mut reg = registers.clone();

        let opcode = program[i];
        let operand = program[i+1];

        match opcode {
            // adv instruction
            0 => {registers.A = reg.A/(2 as i128).pow(get_combo(reg, operand) as u32);},
            // bxl instruction
            1 => {registers.B = reg.B ^ operand;},
            // bst instruction
            2 => {registers.B = get_combo(reg, operand)%8;},
            // jnz instruction
            3 => {
                if(registers.A != 0) {
                    i = operand as usize;
                    continue;
                }
            },
            // bxc instruction
            4 => {registers.B = reg.B ^ reg.C;},
            // out instruction
            5 => {
                let combo = get_combo(reg, operand)%8;
                ret_string += format!("{combo},").as_str();
            },
            // bdv instruction
            6 => {registers.B = reg.A/(2 as i128).pow(get_combo(reg, operand) as u32);},
            // cdv instruction
            7 => {registers.C = reg.A/(2 as i128).pow(get_combo(reg, operand) as u32);},
            _ => println!("unexpected opcode!"),
        }

        i += 2;
    }

    ret_string += "\n";

    return ret_string;
}

fn parse_input(input: &str) -> (Registers, Vec<i128>) {
    let mut registers: Registers = Registers { A: 0, B: 0, C: 0 };

    registers.A = input.lines().nth(0).unwrap().split("Register A: ").nth(1).unwrap().parse::<i128>().unwrap();
    registers.B = input.lines().nth(1).unwrap().split("Register B: ").nth(1).unwrap().parse::<i128>().unwrap();
    registers.C = input.lines().nth(2).unwrap().split("Register C: ").nth(1).unwrap().parse::<i128>().unwrap();

    let program: Vec<i128> = input.lines().nth(4).unwrap().split([' ', ',']).into_iter().skip(1).map(|e| e.parse::<i128>().unwrap()).collect();

    return (registers, program);
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;

    let parsed = parse_input(input);
    let mut registers = parsed.0;
    let mut program = parsed.1;

    let prog_str = do_program(&mut registers, &mut program);

    print!("{prog_str}");

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut start_a: i128 = 0o500000100000004;
    //                      2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0

    let parsed = parse_input(input);
    let mut registers = Registers{A: start_a, B: 0, C:0};
    let mut program = parsed.1;

    let prog_str = do_program(&mut registers, &mut program);

    print!("{prog_str}");

    return start_a;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_day_17_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 11048);
    }

    #[test]
    fn test_day_17_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 78428);
    }

    #[test]
    fn test_day_17_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 81);
    }

    #[test]
    fn test_day_17_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1366);
    }
}
