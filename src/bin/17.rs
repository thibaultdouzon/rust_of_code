use itertools::Itertools;
advent_of_code::solution!(17);

#[derive(Debug, Copy, Clone)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

type Literal = usize;
type Combo = usize;

#[derive(Debug)]
enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc(),
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

fn parse_input(input: &str) -> (Registers, Vec<usize>) {
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let registers_v: Vec<usize> = blocks[0]
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
        .collect();
    let registers = Registers {
        a: registers_v[0],
        b: registers_v[1],
        c: registers_v[2],
    };
    let instructions = blocks[1]
        .split_whitespace()
        .last()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();

    (registers, instructions)
}

fn parse_instructions(instructions: &Vec<usize>) -> Vec<Instruction> {
    instructions
        .windows(2)
        .map(|chunk| match chunk[0] {
            0 => Instruction::Adv(chunk[1]),
            1 => Instruction::Bxl(chunk[1]),
            2 => Instruction::Bst(chunk[1]),
            3 => Instruction::Jnz(chunk[1]),
            4 => Instruction::Bxc(),
            5 => Instruction::Out(chunk[1]),
            6 => Instruction::Bdv(chunk[1]),
            7 => Instruction::Cdv(chunk[1]),
            _ => panic!("Invalid instruction"),
        })
        .collect()
}

fn combo_value(combo: Combo, registers: &Registers) -> usize {
    match combo {
        n if (0..=3).contains(&n) => n,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("Invalid combo"),
    }
}

fn exec(instruction: &Instruction, registers: &mut Registers) -> (Option<usize>, Option<usize>) {
    match instruction {
        Instruction::Adv(n) => {
            let new_a = registers.a >> combo_value(*n, registers);
            registers.a = new_a;
            (None, None)
        }
        Instruction::Bxl(n) => {
            registers.b = registers.b ^ n;
            (None, None)
        }
        Instruction::Bst(n) => {
            registers.b = combo_value(*n, &registers) & 0b111;
            (None, None)
        }
        Instruction::Jnz(n) => {
            if registers.a != 0 {
                (None, Some(*n))
            } else {
                (None, None)
            }
        }
        Instruction::Bxc() => {
            registers.b = registers.b ^ registers.c;
            (None, None)
        }
        Instruction::Out(n) => (Some(combo_value(*n, registers) & 0b111), None),
        Instruction::Bdv(n) => {
            let new_b = registers.a >> combo_value(*n, registers);
            registers.b = new_b;
            (None, None)
        }

        Instruction::Cdv(n) => {
            let new_c = registers.a >> combo_value(*n, registers);
            registers.c = new_c;
            (None, None)
        }
    }
}

fn run_program(registers: &Registers, instructions: &Vec<Instruction>) -> Vec<usize> {
    let mut registers = registers.clone();
    let mut pc = 0;
    let mut out = Vec::new();
    loop {
        if let Some(ins) = instructions.get(pc) {
            let (out_val, jump) = exec(ins, &mut registers);
            if let Some(val) = out_val {
                out.push(val);
            }
            if let Some(jump) = jump {
                pc = jump;
            } else {
                pc += 2;
            }
        } else {
            break;
        }
    }
    out
}

#[allow(dead_code)]
fn check_output(
    registers: &Registers,
    instructions: &Vec<Instruction>,
    desired_out: &Vec<usize>,
) -> usize {
    let mut registers = registers.clone();
    let mut pc = 0;
    let mut out_pointer = 0;
    loop {
        if let Some(ins) = instructions.get(pc) {
            let (out_val, jump) = exec(ins, &mut registers);
            if let Some(val) = out_val {
                if out_pointer == desired_out.len() || val != desired_out[out_pointer] {
                    return out_pointer;
                }
                out_pointer += 1;
            }
            if let Some(jump) = jump {
                pc = jump;
            } else {
                pc += 2;
            }
        } else {
            break;
        }
    }
    out_pointer
}

fn find_a(
    registers: &Registers,
    instructions: &Vec<Instruction>,
    desired_out: &Vec<usize>,
    steps: i32,
) -> Option<usize> {
    if steps < 0 {
        return Some(registers.a);
    }
    for i in 0..8 {
        let this_registers = Registers {
            a: (registers.a << 3) + i,
            b: registers.b,
            c: registers.c,
        };
        let out = run_program(&this_registers, instructions);

        if out[0] == desired_out[steps as usize] {
            let res = find_a(&this_registers, instructions, desired_out, steps - 1);
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    let (registers, instructions) = parse_input(input);
    let instructions = parse_instructions(&instructions);

    let res = run_program(&registers, &instructions);
    Some(res.iter().map(|x| x.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, instructions_v) = parse_input(input);
    let instructions = parse_instructions(&instructions_v);
    let registers = Registers { a: 0, b: 0, c: 0 };
    let res = find_a(
        &registers,
        &instructions,
        &instructions_v,
        instructions_v.len() as i32 - 1,
    );

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
