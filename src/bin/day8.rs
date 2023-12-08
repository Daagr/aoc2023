use std::{collections::HashMap, error::Error, io::BufRead, str::FromStr};

use smol_str::SmolStr;

#[derive(Clone, Debug)]
struct Node {
    name: SmolStr,
    left: SmolStr,
    right: SmolStr,
}

impl FromStr for Node {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once('=').ok_or("no equals signs")?;
        let name = name.trim_end();
        let (left, right) = rest.trim_start().split_once(',').ok_or("no comma")?;
        let left = left.strip_prefix('(').ok_or("no left paren")?;
        let right = right
            .trim_start()
            .strip_suffix(')')
            .ok_or("no right paren")?;
        Ok(Node {
            name: name.into(),
            left: left.into(),
            right: right.into(),
        })
    }
}

fn follow(directions: &str, nodes: &HashMap<SmolStr, Node>, first_part: bool) -> usize {
    let mut current_nodes = Vec::new();
    if first_part {
        current_nodes.push(&nodes["AAA"]);
    } else {
        for (name, node) in nodes {
            if name.ends_with('A') {
                current_nodes.push(node);
            }
        }
    }
    let mut next_nodes;
    for (n, d) in std::iter::repeat(directions.chars()).flatten().enumerate() {
        if current_nodes.iter().all(|n| n.name.ends_with('Z')) {
            return n;
        }
        next_nodes = Vec::with_capacity(current_nodes.len());
        match d {
            'L' => {
                for node in &current_nodes {
                    next_nodes.push(&nodes[&node.left]);
                }
            }
            'R' => {
                for node in &current_nodes {
                    next_nodes.push(&nodes[&node.right]);
                }
            }
            other => panic!("Unknown direction {}", other),
        }
        std::mem::swap(&mut current_nodes, &mut next_nodes);
    }
    unreachable!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let first_part = match std::env::args().nth(2).as_deref(){
        None => true,
        Some("1" | "A" | "a") => true,
        Some("2" | "B" | "b") => false,
        _ => Err("Unknown part")?,
    };
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/day8.txt".into());
    let mut lines = std::io::BufReader::new(std::fs::File::open(&filename)?).lines();
    let directions = lines.next().ok_or("empty file")??;
    lines.next(); // drop empty line
    let mut nodes = HashMap::<SmolStr, _>::new();
    for line in lines {
        let line = line?;
        let node: Node = line.parse()?;
        //println!("{node:?}");
        nodes.insert(node.name.clone(), node);
    }
    let steps = follow(&directions, &nodes, first_part);
    println!("Took {steps} steps");
    return Ok(());
}
