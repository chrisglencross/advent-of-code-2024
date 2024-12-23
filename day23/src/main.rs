use std::collections::{HashMap, HashSet};

use itertools;
use itertools::Itertools;

const DAY: u8 = 23;

fn main() {
    let input = aocutil::load_input(DAY);

    let neighbours = parse_input(&input);
    let nodes: Vec<&str> = neighbours.keys().map(|k| *k).sorted().collect();

    println!("Part 1: {}", count_triple_parties(&neighbours, &nodes));
    println!("Part 2: {}", find_biggest_party(&Vec::new(), &neighbours, &nodes).iter().sorted().join(","));
}

fn count_triple_parties<'a>(neighbours: &HashMap<&str, HashSet<&str>>, nodes: &[&'a str]) -> i64 {
    let mut total = 0;
    for (i, &n1) in nodes[0..].iter().enumerate() {
        for (j, &n2) in nodes[i + 1..].iter().enumerate() {
            if can_add_to_party(&[n1], n2, neighbours) {
                for &n3 in nodes[i + j + 1..].iter() {
                    if can_add_to_party(&[n1, n2], n3, neighbours) {
                        if n1.starts_with('t') || n2.starts_with('t') || n3.starts_with('t') {
                            total += 1;
                        }
                    }
                }
            }
        }
    }
    total
}

fn find_biggest_party<'a>(party: &Vec<&'a str>, neighbours: &HashMap<&str, HashSet<&str>>, remaining: &[&'a str]) -> Vec<&'a str> {
    let mut biggest_party: Vec<&str> = party.clone();
    for (i, candidate) in remaining.iter().enumerate() {
        if can_add_to_party(party, candidate, neighbours) {
            let mut new_party = party.clone();
            new_party.push(candidate);
            let new_biggest_party = find_biggest_party(&new_party, neighbours, &remaining[i + 1..]);
            if new_biggest_party.len() > biggest_party.len() {
                biggest_party = new_biggest_party;
            }
        }
    }
    biggest_party
}

fn can_add_to_party(party: &[&str], candidate: &str, neighbours: &HashMap<&str, HashSet<&str>>) -> bool {
    let candidate_neighbours = &neighbours[candidate];
    party.iter().all(|party_member| candidate_neighbours.contains(party_member))
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut result = HashMap::new();
    input.lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(n1, n2)| {
            result.entry(n1).or_insert(HashSet::new()).insert(n2);
            result.entry(n2).or_insert(HashSet::new()).insert(n1);
        });
    result
}