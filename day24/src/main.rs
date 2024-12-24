use std::collections::HashMap;
use std::fmt::format;
use std::ops::Deref;

use itertools;
use itertools::Itertools;

const DAY: u8 = 24;

fn main() {
    let input = aocutil::load_input(DAY);
    let (inputs, gates) = parse_input(&input);

    let part1 = gates.keys().filter(|k| k.starts_with("z")).sorted().rev()
        .fold(0, |acc, gate| (acc << 1) + evaluate(gate, &gates, &inputs));
    println!("Part 1: {part1}");

    // let mut inputs = HashMap::new();
    // for i in 0u8..45u8 {
    //     inputs.insert(format!("x{i:02}"), 1);
    //     inputs.insert(format!("y{i:02}"), 0);
    // }

    // let part1 = u64::from_str_radix(&z_gates.iter().map(|(_, b)| b.to_string()).join(""), 2).unwrap();
    // println!("Part 1: {part1}");


    // for gate in gates.keys() {
    //     if gate.starts_with("z") {
    //         let value = evaluate(gate, &gates, &inputs);
    //         z_gates.push((gate, value));
    //     }
    // }
    // z_gates.sort();
    // z_gates.reverse();
    // let part1 = u64::from_str_radix(&z_gates.iter().map(|(_, b)| b.to_string()).join(""), 2).unwrap();
    // println!("Part 1: {part1}");

    let mut labels: HashMap<&String, String> = HashMap::new();
    for (gate, (i1, op, i2)) in &gates {
        if (i1.starts_with("x") && i2.starts_with("y")) || (i1.starts_with("y") && i2.starts_with("x")) {
            let bit1 = &i1[1..];
            let bit2 = &i2[1..];
            if bit1 != bit2 {
                panic!();
            }
            if op == "AND" {
                labels.insert(gate, format!("{bit1}: Input AND"));
            } else if op == "XOR" {
                if gate.starts_with("z") {
                    // First is only a half adder
                    labels.insert(gate, format!("{bit1}: Output XOR"));
                } else {
                    labels.insert(gate, format!("{bit1}: Input XOR"));
                }
            } else {
                panic!();
            }
        } else if i1.starts_with("x") || i2.starts_with("x") {
            panic!();
        } else if i1.starts_with("y") || i2.starts_with("y") {
            panic!();
        }
    }

    for (gate, (i1, op, i2)) in &gates {
        if let Some(label) = labels.get(i1) {
            let label = String::from(label);
            if op == "XOR" && label.ends_with(": Input XOR") {
                let bit = label.split_once(":").unwrap().0;
                labels.insert(gate, format!("{bit}: Output XOR"));
                if !labels.contains_key(i2) {
                    let previous_bit = bit.parse::<u8>().unwrap() - 1;
                    let previous_op = &gates.get(i2).unwrap().1;
                    if previous_op != "OR" {
                        println!("Error: carry out signal {i2} from {previous_bit} is an {previous_op} gate. Should be OR");
                    }
                    labels.insert(i2, format!("{previous_bit}: Carry Out {previous_op}"));
                }
            } else if op == "AND" && label.ends_with(": Input XOR") {
                let bit = label.split_once(":").unwrap().0;
                labels.insert(gate, format!("{bit}: Output AND"));
            }
        }
        if let Some(label) = labels.get(i2) {
            let label = String::from(label);
            if op == "XOR" && label.ends_with(": Input XOR") {
                let bit = label.split_once(":").unwrap().0;
                labels.insert(gate, format!("{bit}: Output XOR"));
                if !labels.contains_key(i1) {
                    let previous_bit = bit.parse::<u8>().unwrap() - 1;
                    let previous_op = &gates.get(i1).unwrap().1;
                    if previous_op != "OR" {
                        println!("Error: carry out signal {i1} from {previous_bit} is an {previous_op} gate. Should be OR");
                    }
                    labels.insert(i1, format!("{previous_bit}: Carry Out {previous_op}"));
                }
            } else if op == "AND" && label.ends_with(": Input XOR") {
                let bit = label.split_once(":").unwrap().0;
                labels.insert(gate, format!("{bit}: Output AND"));
            }
        }
    }

    for (gate, (i1, op, i2)) in &gates {
        if op == "OR" {
            if let Some(label1) = labels.get(i1) {
                let label1 = String::from(label1);
                if let Some(label2) = labels.get(i2) {
                    let label2 = String::from(label2);
                    let bit1 = label1.split_once(":").unwrap().0;
                    let bit2 = label2.split_once(":").unwrap().0;
                    if bit1 != bit2 {
                        panic!();
                    }
                    labels.insert(gate, format!("{bit1}: Carry Out OR"));
                } else {
                    println!("No label found for {i2}");
                }
            } else {
                println!("No label found for {i1}");
            }
        }
    }

    println!("digraph G {{");
    for (gate, (i1, op, i2)) in &gates {
        println!("\t{i1} -> {gate};");
        println!("\t{i2} -> {gate};");
        let unknown = String::from(format!("UNKNOWN {}", op));
        let label = labels.get(&gate).unwrap_or(&unknown);
        println!("\t{gate}[label=\"{label}\\n{gate}\"];\n");
    }

    println!("}}");


        let part2: i64 = 0;
    println!("Part 2: {part2}");
}

fn evaluate(gate: &String, gates: &HashMap<String, (String, String, String)>, inputs: &HashMap<String, u8>) -> u64 {
    if let Some(&input) = inputs.get(gate) {
        input as u64
    } else if let Some((i1, op, i2)) = gates.get(gate) {
        let i1 = evaluate(i1, gates, inputs);
        let i2 = evaluate(i2, gates, inputs);
        match op.deref() {
            "AND" => i1 & i2,
            "OR" => i1 | i2,
            "XOR" => i1 ^ i2,
            _ => panic!()
        }
    } else {
        panic!()
    }
}

fn parse_input(input: &str) -> (HashMap<String, u8>, HashMap<String, (String, String, String)>) {
    let (block1, block2) = input.split_once("\n\n").unwrap();

    let inputs: HashMap<String, u8> = block1.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|p| (String::from(p.0), p.1.parse().unwrap()))
        .collect();

    let gates: HashMap<String, (String, String, String)> = block2.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| (String::from(parts[4]), (String::from(parts[0]), String::from(parts[1]), String::from(parts[2]))))
        .collect();

    (inputs, gates)
}