use regex::Regex;

const DAY: u8 = 17;

const ADV: i64 = 0;
const BXL: i64 = 1;
const BST: i64 = 2;
const JNZ: i64 = 3;
const BXC: i64 = 4;
const OUT: i64 = 5;
const BDV: i64 = 6;
const CDV: i64 = 7;

fn main() {
    let input = aocutil::load_input(DAY);
    let (registers, program) = parse_input(&input);

    let output = run(registers, &program);

    println!("Part 1: {}", output.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","));
    println!("Part 2: {}", find_self_output(registers, &program));

}

fn run(registers: (i64, i64, i64), program: &Vec<i64>) -> Vec<i64> {
    let (mut a, mut b, mut c) = registers;
    let mut pc = 0;
    let mut output = vec![];
    while pc < program.len() {
        let instruction = program[pc];
        let arg = program[pc + 1];
        pc += 2;

        match instruction {
            ADV => a = a >> combo(arg, a, b, c),
            BXL => b ^= arg,
            BST => b = combo(arg, a, b, c) % 8,
            JNZ => if a != 0 { pc = usize::try_from(arg).unwrap(); },
            BXC => b = b ^ c,
            OUT => output.push(combo(arg, a, b, c) % 8),
            BDV => b = a >> combo(arg, a, b, c),
            CDV => c = a >> combo(arg, a, b, c),
            _ => panic!(),
        }
    }
    output
}

fn combo(op: i64, a: i64, b: i64, c: i64) -> i64 {
    match op {
        0..=3 => op,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!()
    }
}

/// Decompiled `input.txt` manually. This function is not used in the solution.
/// The main takeaway from decompiling is that each value of output is derived from the least
/// significant remaining 10 bits of register `a`, and the bottom 3 bits of `a` are thrown away on
/// each iteration. 3 bit chunks mean they can be considered octal digits, with 4 octal digits
/// affecting each output value.
///
/// This is useful to know for part 2.
#[allow(dead_code)]
fn run_hardcoded(registers: (i64, i64, i64)) -> Vec<i64> {
    let (mut a, _, _) = registers;
    let mut output: Vec<i64> = vec![];
    while a != 0 {
        let mut b = a % 8;     // BST 4 | take last 3 bits of a
        b = b ^ 2;                  // BXL 2 | toggle second bit
        let c = a >> b;        // CDV 5 | take 3 higher bits of a (could be up to the 10th bit)
        b = b ^ 7;                  // BXL 7 | b = not b
        b = b ^ c;                  // BXC 4 | b = b xor c
        a = a >> 3;                 // ADV 3 | shift to next 3 bit chunk of a
        output.push(b % 8);   // OUT 5 | output b last 3 bits
    }
    output
}

fn find_self_output(registers: (i64, i64, i64), program: &Vec<i64>) -> i64 {
    let program_rev: Vec<i64> = program.iter().rev().map(|n| *n).collect();

    let mut octal_a: [i64; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut correct_octal_digits = 0;
    let mut counter = 0;
    loop {

        // As discovered from reverse engineering input.txt:
        //
        // Each value of output is derived from up to 10 bits of the input register `a`, with
        // the most significant bits of `a` affecting the last values of output.
        //
        // Conversely, the first octal digit of `a` contributes to the final 4 output values (0 to 3 from the end).
        // First and second octal digits of `a` may contribute to output values 1 to 4 from the end, etc.
        //
        // We therefore need to find the first octal digit of `a` before we try to find the second digit,
        // and in general discover the most significant digits first. This also helps ensure that we find
        // the lowest possible solution value.

        octal_a[correct_octal_digits + 0] = (counter >> 9) % 8;
        octal_a[correct_octal_digits + 1] = (counter >> 6) % 8;
        octal_a[correct_octal_digits + 2] = (counter >> 3) % 8;
        octal_a[correct_octal_digits + 3] = (counter >> 0) % 8;

        // We need a non-zero value in most significant digit to get correct length output
        if octal_a[0] == 0 {
            octal_a[0] = 1;
        }

        let a = octal_a.iter().fold(0i64, |a, n| (a << 3) + n);

        let output = run((a, registers.1, registers.2), &program);
        if output == *program {
            return a
        }

        // Check if we found any more digits of correct output at the end. Each octal digit can
        // contribute towards 4 values of output.
        let output_rev: Vec<i64> = output.iter().rev().map(|n| *n).collect();
        if output_rev[correct_octal_digits..=correct_octal_digits + 3] == program_rev[correct_octal_digits..=correct_octal_digits + 3] {
            correct_octal_digits += 1;
            counter = 0;
        } else {
            counter += 1                                    ;
        }

    }
}

fn parse_input(input: &str) -> ((i64, i64, i64), Vec<i64>) {
    let re = Regex::new(r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: (.*)").unwrap();

    let (_, [a, b, c, program]) = re.captures(input).unwrap().extract();
    (
        (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()),
        program.split(",").map(|p| p.parse().unwrap()).collect()
    )
}