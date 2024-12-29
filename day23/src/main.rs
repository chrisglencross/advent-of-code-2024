use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const DAY: u8 = 23;

fn main() {
    let input = aocutil::load_input(DAY);

    let connections = parse_input(&input);
    let computers: Vec<&str> = connections.keys().copied().sorted().collect();

    println!("Part 1: {}", count_triple_parties(&computers, &connections));
    println!("Part 2: {}", find_biggest_party(Vec::new(), &connections, &computers).iter().sorted().join(","));
}

fn count_triple_parties(computers: &[&str], connections: &HashMap<&str, HashSet<&str>>) -> i64 {
    let mut total = 0;
    for (i, &n1) in computers[0..].iter().enumerate() {
        for (j, &n2) in computers[i + 1..].iter().enumerate() {
            if can_add_to_party(&[n1], n2, connections) {
                for &n3 in computers[i + j + 1..].iter() {
                    if can_add_to_party(&[n1, n2], n3, connections) && (n1.starts_with('t') || n2.starts_with('t') || n3.starts_with('t')) {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}

fn find_biggest_party<'a>(party: Vec<&'a str>, connections: &HashMap<&str, HashSet<&str>>, remaining_computers: &[&'a str]) -> Vec<&'a str> {
    remaining_computers.iter().enumerate()
        .filter(|&(_i, candidate)| can_add_to_party(&party, candidate, connections))
        .map(|(i, candidate)| {
            let new_party = party.iter().chain([candidate]).cloned().collect();
            find_biggest_party(new_party, connections, &remaining_computers[i + 1..])
        })
        .max_by_key(|p| p.len())
        .unwrap_or(party)
}

fn can_add_to_party(party: &[&str], candidate: &str, connections: &HashMap<&str, HashSet<&str>>) -> bool {
    let candidate_neighbours = &connections[candidate];
    party.iter().all(|party_member| candidate_neighbours.contains(party_member))
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut connections = HashMap::new();
    input.lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(n1, n2)| {
            connections.entry(n1).or_insert(HashSet::new()).insert(n2);
            connections.entry(n2).or_insert(HashSet::new()).insert(n1);
        });
    connections
}