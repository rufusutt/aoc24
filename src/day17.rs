const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

fn combo(reg: &[u64; 3], n: u8) -> u64 {
    match n {
        0..=3 => n as u64,
        4..=6 => reg[n as usize - 4],
        _ => panic!("Invalid combo operand"),
    }
}

fn run(mut reg: [u64; 3], program: &[u8], out: &mut Vec<u8>) {
    let mut pc = 0;

    while pc < program.len() {
        let opcode = program[pc];
        let operand = program[pc + 1];

        match opcode {
            ADV => {
                reg[0] = reg[0] / 2u64.pow(combo(&reg, operand) as u32);
                pc += 2;
            }
            BXL => {
                reg[1] = reg[1] ^ operand as u64;
                pc += 2;
            }
            BST => {
                reg[1] = combo(&reg, operand) % 8;
                pc += 2;
            }
            JNZ => {
                if reg[0] > 0 {
                    pc = operand as usize;
                } else {
                    pc += 2;
                }
            }
            BXC => {
                reg[1] = reg[1] ^ reg[2];
                pc += 2;
            }
            OUT => {
                out.push((combo(&reg, operand) % 8) as u8);
                pc += 2;
            }
            BDV => {
                reg[1] = reg[0] / 2u64.pow(combo(&reg, operand) as u32);
                pc += 2;
            }
            CDV => {
                reg[2] = reg[0] / 2u64.pow(combo(&reg, operand) as u32);
                pc += 2;
            }
            _ => panic!("Invalid opcode"),
        }
    }
}

pub fn solution(input: &str) {
    let (reg, program) = input.split_once("\n\n").unwrap();

    let mut reg_iter = reg
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|val| val.parse::<u64>().unwrap());
    let mut registers: [u64; 3] = std::array::from_fn(|_| reg_iter.next().unwrap());

    let program: Vec<_> = program
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|op| op.parse::<u8>().unwrap())
        .collect();

    let mut out = Vec::new();
    run(registers, &program, &mut out);

    println!(
        "Part 1: {}",
        out.iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    // TODO: Should reverse engineer the program to solve this properly
    let mut total = 0;
    for (i, n) in program.iter().rev().enumerate() {
        println!("Digit: {}", i);

        total <<= 3;
        registers[0] = total;

        loop {
            out.clear();
            run(registers, &program, &mut out);
            if out[0] == *n {
                break;
            }

            total += 1;
            registers[0] = total;
        }
    }
    println!("Part 2: {}", total);
}
