use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;

use itertools;
use itertools::Itertools;

const DAY: u8 = 24;

fn main() -> std::io::Result<()> {
    let input = aocutil::load_input(DAY);
    let (inputs, gates) = parse_input(&input);

    let part1 = gates.keys().filter(|k| k.starts_with("z")).sorted().rev()
        .fold(0, |acc, gate| (acc << 1) + evaluate(gate, &gates, &inputs));
    println!("Part 1: {part1}");

    let path = generate_diagram(&gates)?;
    println!("Part 2: See diagram at {path}");

    return Ok(());
}

fn generate_diagram(gates: &HashMap<String, (String, String, String)>) -> std::io::Result<String> {

    enum AdderGate {
        XOR1(u8),
        XOR2(u8),
        AND1(u8),
        AND2(u8),
        OR(u8),  // Carry out
        ERROR(String)
    }

    let mut labels: HashMap<&String, AdderGate> = HashMap::new();

    // Identify XOR1 and AND1 gates connected to inputs
    for (gate, (i1, op, i2)) in gates {
        let x = [i1, i2].into_iter().find(|g| g.starts_with("x"));
        let y = [i1, i2].into_iter().find(|g| g.starts_with("y"));
        if let (Some(x), Some(y)) = (x, y) {
            let xbit = &x[1..];
            let ybit= &y[1..];
            if xbit != ybit {
                labels.insert(gate, AdderGate::ERROR(String::from(format!("Input connections for different bits {x} and {y}"))));
            } else if op == "XOR" {
                labels.insert(gate, AdderGate::XOR1(xbit.parse().unwrap()));
            } else if op == "AND" {
                labels.insert(gate, AdderGate::AND1(xbit.parse().unwrap()));
            } else {
                labels.insert(gate, AdderGate::ERROR(String::from(format!("Inputs should be connected to AND and XOR, not {op}"))));
            }
        } else if x.is_some() || y.is_some() {
            labels.insert(gate, AdderGate::ERROR(String::from("Single input connection")));
        }
    }

    // Find XOR and AND gates not labelled as XOR1 AND AND1
    // Assume these are XOR2 and AND2 and are wired up with inputs from those
    for (gate, (i1, op, i2)) in gates {
        let label = labels.get(gate);
        if label.is_none() {
            // AND2 should have XOR1 and OR(bit-1) as inputs
            let bit = if let Some(AdderGate::XOR1(input_bit)) = labels.get(i1) {
                Some(*input_bit)
            } else if let Some(AdderGate::XOR1(input_bit)) = labels.get(i2) {
                Some(*input_bit)
            } else {
                None
            };
            if let Some(bit) = bit {
                if op == "AND" {
                    labels.insert(gate, AdderGate::AND2(bit));
                } else if op == "XOR" {
                    labels.insert(gate, AdderGate::XOR2(bit));
                } else {
                    labels.insert(gate, AdderGate::ERROR(String::from(format!("Gate has unexpected input from XOR1"))));
                }
            } else if op == "AND" || op == "XOR" {
                labels.insert(gate, AdderGate::ERROR(String::from(format!("Gate should have an input from XOR1"))));
            }
        }
    }

    // Find OR gates and label, also checking that inputs are from AND1 and AND2
    for (gate, (i1, op, i2)) in gates {
        let label = labels.get(gate);
        if label.is_none() {
            // OR should have AND1 as inputs
            let and1_bit = if let Some(AdderGate::AND1(input_bit)) = labels.get(i1) {
                Some(*input_bit)
            } else if let Some(AdderGate::AND1(input_bit)) = labels.get(i2) {
                Some(*input_bit)
            } else {
                None
            };
            let and2_bit = if let Some(AdderGate::AND2(input_bit)) = labels.get(i1) {
                Some(*input_bit)
            } else if let Some(AdderGate::AND2(input_bit)) = labels.get(i2) {
                Some(*input_bit)
            } else {
                None
            };
            if op == "OR" {
                if and1_bit.is_none() || and2_bit.is_none() {
                    labels.insert(gate, AdderGate::ERROR(String::from(format!("Gate should have an input from both AND1 and AND2"))));
                } else {
                    labels.insert(gate, AdderGate::OR(and2_bit.unwrap()));
                }
            } else {
                labels.insert(gate, AdderGate::ERROR(String::from(format!("Unrecognised gate"))));
            }
        }
    }

    // Verify that XOR2 and AND2 both have inputs from carry OR(bit-1)
    for (gate, (i1, _, i2)) in gates {
        let self_bit = if let Some(AdderGate::XOR2(input_bit)) = labels.get(gate) {
            Some(*input_bit)
        } else if let Some(AdderGate::AND2(input_bit)) = labels.get(gate) {
            Some(*input_bit)
        } else {
            None
        };
        let or_bit = if let Some(AdderGate::OR(input_bit)) = labels.get(i1) {
            Some(*input_bit)
        } else if let Some(AdderGate::OR(input_bit)) = labels.get(i2) {
            Some(*input_bit)
        } else {
            None
        };
        if let Some(self_bit) = self_bit {
            if let Some(or_bit) = or_bit {
                if self_bit != or_bit + 1 {
                    labels.insert(gate, AdderGate::ERROR(String::from("Carry bit input is from incorrect preceding adder")));
                }
            } else if self_bit > 1 {
                labels.insert(gate, AdderGate::ERROR(String::from("Expected carry bit input from previous adder")));
            }
        }
    }

    // Verify that 'z' output bits are XOR2
    for (gate, _) in gates {
        let is_output = gate.starts_with("z");
        if let Some(AdderGate::XOR2(_)) = labels.get(gate) {
            if !is_output {
                labels.insert(gate, AdderGate::ERROR(String::from("XOR2 gates should be named with a 'z'")));
            }
        } else if is_output && gate != "z00" && gate != "z01" {
            labels.insert(gate, AdderGate::ERROR(String::from(format!("Output gate {gate} named with a 'z' should be XOR2"))));
        }
    }

    let path = "day24/graphviz.dot";
    let mut output = File::create(path)?;
    writeln!(output, "digraph G {{")?;
    for (gate, (i1, op, i2)) in gates {
        writeln!(output, "\t{i1} -> {gate};")?;
        writeln!(output, "\t{i2} -> {gate};")?;
        let (label, color) = match labels.get(gate) {
            None => (String::from("UNRECOGNIZED"), "red"),
            Some(AdderGate:: XOR1(bit)) => (String::from(format!("XOR1 bit {bit}")), "black"),
            Some(AdderGate:: XOR2(bit)) => (String::from(format!("XOR2 bit {bit}")), "black"),
            Some(AdderGate:: AND1(bit)) => (String::from(format!("AND1 bit {bit}")), "black"),
            Some(AdderGate:: AND2(bit)) => (String::from(format!("AND2 bit {bit}")), "black"),
            Some(AdderGate:: OR(bit)) => (String::from(format!("CARRY bit {bit}")), "black"),
            Some(AdderGate::ERROR(message)) => (String::from(format!("{op}: {message}")), "red"),
        };
        writeln!(output, "\t{gate}[color=\"{color}\" label=\"{label}\\n{gate}\"];\n")?;
    }
    writeln!(output, "}}")?;

    Ok(String::from(path))
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