use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;

use itertools::Itertools;

const DAY: u8 = 24;

type Inputs = HashMap<String, u8>;
type Gates = HashMap<String, (String, String, String)>;

fn main() -> std::io::Result<()> {
    let input = aocutil::load_input(DAY);
    let (inputs, gates) = parse_input(&input);

    let part1 = gates.keys().filter(|k| k.starts_with("z")).sorted().rev()
        .fold(0, |acc, gate| (acc << 1) + evaluate(gate, &gates, &inputs));
    println!("Part 1: {part1}");

    let path = generate_diagram(&gates)?;
    println!("Part 2: See diagram at {path}. Inspect regions highlighted in red (or just above) to work out which connections to swap. The errors are localised.");
    println!("For each single bit adder with an error confirm that:");
    println!("  1. (x, y) -> XOR1");
    println!("  2. (x, y) -> AND1");
    println!("  3. (XOR1, Carry In)) -> XOR2 (aka. OUTPUT)");
    println!("  4. (XOR1, Carry In)) -> AND2");
    println!("  5. (AND1, AND2) -> OR (aka. CARRY)");

    Ok(())
}

/// Draw a graph of the gates, highlighting errors in red where this is not a standard full adder.
fn generate_diagram(gates: &Gates) -> std::io::Result<String> {

    enum GateInfo {
        XOR1(u8),
        XOR2(u8),
        AND1(u8),
        AND2(u8),
        OR(u8), // aka. "Carry out"
        Error(String)
    }

    let mut labels: HashMap<&String, GateInfo> = HashMap::new();

    // Identify XOR1 and AND1 gates connected to inputs
    for (gate, (i1, op, i2)) in gates {
        let x = [i1, i2].into_iter().find(|g| g.starts_with("x"));
        let y = [i1, i2].into_iter().find(|g| g.starts_with("y"));
        if let (Some(x), Some(y)) = (x, y) {
            let xbit = &x[1..];
            let ybit= &y[1..];
            if xbit != ybit {
                labels.insert(gate, GateInfo::Error(format!("Input connections for different bits {x} and {y}")));
            } else if op == "XOR" {
                labels.insert(gate, GateInfo::XOR1(xbit.parse().unwrap()));
            } else if op == "AND" {
                labels.insert(gate, GateInfo::AND1(xbit.parse().unwrap()));
            } else {
                labels.insert(gate, GateInfo::Error(format!("Inputs should be connected to AND and XOR, not {op}")));
            }
        } else if x.is_some() || y.is_some() {
            labels.insert(gate, GateInfo::Error(String::from("Single input connection")));
        }
    }

    // Find XOR and AND gates not labelled as XOR1 AND AND1
    // Assume these are XOR2 and AND2 and are wired up with inputs from those
    for (gate, (i1, op, i2)) in gates {
        let label = labels.get(gate);
        if label.is_none() {
            // AND2 should have XOR1 and OR(bit-1) as inputs
            let bit = if let Some(GateInfo::XOR1(input_bit)) = labels.get(i1) {
                Some(*input_bit)
            } else if let Some(GateInfo::XOR1(input_bit)) = labels.get(i2) {
                Some(*input_bit)
            } else {
                None
            };
            if let Some(bit) = bit {
                if op == "AND" {
                    labels.insert(gate, GateInfo::AND2(bit));
                } else if op == "XOR" {
                    labels.insert(gate, GateInfo::XOR2(bit));
                } else {
                    labels.insert(gate, GateInfo::Error("Gate has unexpected input from XOR1".to_string()));
                }
            } else if op == "AND" || op == "XOR" {
                labels.insert(gate, GateInfo::Error("Gate should have an input from XOR1".to_string()));
            }
        }
    }

    // Find OR gates and label, also checking that inputs are from AND1 and AND2
    for (gate, (i1, op, i2)) in gates {
        let label = labels.get(gate);
        if label.is_none() {
            // OR should have AND1 as inputs
            let and1_bit = if let Some(GateInfo::AND1(input_bit)) = labels.get(i1) {
                Some(*input_bit)
            } else if let Some(GateInfo::AND1(input_bit)) = labels.get(i2) {
                Some(*input_bit)
            } else {
                None
            };
            let and2_bit = if let Some(GateInfo::AND2(input_bit)) = labels.get(i1) {
                Some(*input_bit)
            } else if let Some(GateInfo::AND2(input_bit)) = labels.get(i2) {
                Some(*input_bit)
            } else {
                None
            };
            if op == "OR" {
                if and1_bit.is_none() || and2_bit.is_none() {
                    labels.insert(gate, GateInfo::Error("Gate should have an input from both AND1 and AND2".to_string()));
                } else {
                    labels.insert(gate, GateInfo::OR(and2_bit.unwrap()));
                }
            } else {
                labels.insert(gate, GateInfo::Error("Unrecognised gate".to_string()));
            }
        }
    }

    // Verify that XOR2 and AND2 both have inputs from carry OR(bit-1)
    for (gate, (i1, _, i2)) in gates {
        let self_bit = if let Some(GateInfo::XOR2(input_bit)) = labels.get(gate) {
            Some(*input_bit)
        } else if let Some(GateInfo::AND2(input_bit)) = labels.get(gate) {
            Some(*input_bit)
        } else {
            None
        };
        let or_bit = if let Some(GateInfo::OR(input_bit)) = labels.get(i1) {
            Some(*input_bit)
        } else if let Some(GateInfo::OR(input_bit)) = labels.get(i2) {
            Some(*input_bit)
        } else {
            None
        };
        if let Some(self_bit) = self_bit {
            if let Some(or_bit) = or_bit {
                if self_bit != or_bit + 1 {
                    labels.insert(gate, GateInfo::Error(String::from("Carry bit input is from incorrect preceding adder")));
                }
            } else if self_bit > 1 {
                labels.insert(gate, GateInfo::Error(String::from("Expected carry bit input from previous adder")));
            }
        }
    }

    // Verify that 'z' output bits are XOR2
    for gate in gates.keys() {
        let is_output = gate.starts_with("z");
        if let Some(GateInfo::XOR2(_)) = labels.get(gate) {
            if !is_output {
                labels.insert(gate, GateInfo::Error(String::from("OUTPUT (XOR2) gates should be named with a 'z'")));
            }
        } else if is_output && gate != "z00" && gate != "z01" {
            labels.insert(gate, GateInfo::Error(format!("Gate {gate} named with a 'z' should be OUTPUT (XOR2)")));
        }
    }

    let path = "day24/graphviz.dot";
    let mut output = File::create(path)?;
    writeln!(output, "digraph G {{")?;
    for (gate, (i1, op, i2)) in gates.iter().sorted_by_key(|&(gate, (_, op, _))| (op.clone(), gate.clone())) {
        writeln!(output, "\t{i1} -> {gate};")?;
        writeln!(output, "\t{i2} -> {gate};")?;
        let (label, color, shape) = match labels.get(gate) {
            None => (String::from("UNRECOGNIZED"), "red", "box"),
            Some(GateInfo:: XOR1(bit)) => (format!("XOR1 bit {bit}"), "black", "diamond"),
            Some(GateInfo:: XOR2(bit)) => (format!("OUTPUT bit {bit}"), "green", "oval"),
            Some(GateInfo:: AND1(bit)) => (format!("AND1 bit {bit}"), "black", "doubleoctagon"),
            Some(GateInfo:: AND2(bit)) => (format!("AND2 bit {bit}"), "black", "doubleoctagon"),
            Some(GateInfo:: OR(bit)) => (format!("CARRY bit {bit}"), "blue", "hexagon"),
            Some(GateInfo::Error(message)) => (message.to_string(), "red", "box"),
        };
        writeln!(output, "\t{gate}[color=\"{color}\" shape=\"{shape}\" label=\"{label}\\n{op} {gate}\"];\n")?;
    }
    writeln!(output, "}}")?;

    Ok(String::from(path))
}

fn evaluate(gate: &String, gates: &Gates, inputs: &Inputs) -> u64 {
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

fn parse_input(input: &str) -> (Inputs, Gates) {
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