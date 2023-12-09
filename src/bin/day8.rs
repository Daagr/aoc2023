use std::{collections::HashMap, error::Error, io::BufRead, str::FromStr};

#[derive(Clone, Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
    left_id: usize,
    right_id: usize,
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
            left_id: 0,
            right_id: 0,
        })
    }
}

fn next_z(
    directions: &str,
    nodes: &Vec<Node>,
    mut id: usize,
    direction_offset: usize,
) -> (usize, usize) {
    let mut node = &nodes[id];
    for (n, d) in std::iter::repeat(directions.chars())
        .flatten()
        .skip(direction_offset % directions.len())
        .enumerate()
    {
        // if n < 10 {
        //     println!("{n}, {d}, {id}, {node:?}");
        // }
        if n != 0 && node.name.ends_with('Z') {
            return (n, id);
        }
        match d {
            'L' => id = node.left_id,
            'R' => id = node.right_id,
            other => panic!("Unknown direction {}", other),
        }
        node = &nodes[id]
    }
    unreachable!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let first_part = match std::env::args().nth(2).as_deref() {
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
    let mut nodes = Vec::new();
    let mut node_names_to_id = HashMap::new();
    for line in lines {
        let line = line?;
        let node: Node = line.parse()?;
        node_names_to_id.insert(node.name.clone(), nodes.len());
        nodes.push(node);
    }
    for node in nodes.iter_mut() {
        node.left_id = node_names_to_id[&node.left];
        node.right_id = node_names_to_id[&node.right];
    }
    //let steps = follow(&directions, &nodes, first_part);
    let (steps, z_id) = next_z(&directions, &nodes, node_names_to_id["AAA"], 0);
    println!(
        "Took {steps} steps to find {:?} at {z_id} out of {}",
        nodes[z_id],
        nodes.len()
    );

    let (nsteps, nz_id) = next_z(&directions, &nodes, z_id, steps);
    println!(
        "Then it took {nsteps} steps to find {:?} at {nz_id}",
        nodes[nz_id]
    );
    return Ok(());
}
