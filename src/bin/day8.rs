use std::{io::BufRead, str::FromStr, error::Error, collections::HashMap};

// TODO: consider a type with small string optimization or some kind of interned string type
#[derive(Clone, Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node{
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let (name, rest) = s.split_once('=').ok_or("no equals signs")?;
        let name = name.trim_end();
        let (left, right) = rest.trim_start().split_once(',').ok_or("no comma")?;
        let left = left.strip_prefix('(').ok_or("no left paren")?;
        let right = right.trim_start().strip_suffix(')').ok_or("no right paren")?;
        Ok(Node{ name: name.into(), left: left.into(), right: right.into() })
    }
}

fn follow(directions: &str, nodes: &HashMap<String, Node>) -> usize{
    let mut node = &nodes["AAA"];
    for (n, d) in std::iter::repeat(directions.chars()).flatten().enumerate(){
        if node.name == "ZZZ"{
            return n;
        }
        match d{
            'L' => node = &nodes[&node.left],
            'R' => node = &nodes[&node.right],
            other => panic!("Unknown direction {}", other)
        }
    }
    unreachable!();
}

fn main() -> Result<(), Box<dyn Error>>{
    let filename = std::env::args().nth(1).unwrap_or_else(||"data/day8.txt".into());
    let mut lines = std::io::BufReader::new(std::fs::File::open(&filename)?).lines();
    let directions = lines.next().ok_or("empty file")??;
    lines.next(); // drop empty line
    let mut nodes = HashMap::<String, _>::new();
    for line in lines{
        let line = line?;
        let node: Node = line.parse()?;
        //println!("{node:?}");
        nodes.insert(node.name.clone(), node);
    }
    let steps = follow(&directions, &nodes);
    println!("Took {steps} steps");
    return Ok(());
}