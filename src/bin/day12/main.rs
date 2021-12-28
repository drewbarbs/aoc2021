use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
struct Node {
    name: String,
    is_small: bool,
    outgoing: HashSet<String>,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph {
        nodes: HashMap::new(),
    };
    for l in input.lines() {
        let parts = l.split('-').collect::<Vec<_>>();
        let from = graph.nodes.entry(parts[0].into()).or_insert(Node {
            name: parts[0].into(),
            is_small: parts[0].chars().all(|c| c.is_lowercase()),
            outgoing: HashSet::new(),
        });
        from.outgoing.insert(parts[1].into());

        let to = graph.nodes.entry(parts[1].into()).or_insert(Node {
            name: parts[0].into(),
            is_small: parts[0].chars().all(|c| c.is_lowercase()),
            outgoing: HashSet::new(),
        });
        to.outgoing.insert(parts[0].into());
    }

    graph
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let graph = parse_input(&input);

    println!("{:?}", graph);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const SAMPLE_INPUT: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn test_deserialize() {
        let graph = parse_input(SAMPLE_INPUT);
        let mut node_names = graph.nodes.iter().map(|(k, _v)| k).collect::<Vec<_>>();
        node_names.sort();

        assert_eq!(node_names, vec!["A", "b", "c", "d", "end", "start"]);
    }
}
