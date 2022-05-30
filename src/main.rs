use std::io;

struct Node {
    prev_nodes: Vec<usize>,
    next_nodes: Vec<usize>
}

impl Node {
    fn new() -> Node {
        Node { prev_nodes: vec![], next_nodes: vec![] }
    }
}

fn main() {
    println!("Hello, world!");
}

fn get_puzzle_answer(nodes: &Vec<Node>) -> usize {

}

fn nodes_from_edges(edges: &Vec<Vec<usize>>) -> Vec<Node> {
    let &max_node_ind = edges.iter()
        .flatten()
        .max()
        .unwrap();
    
    let mut nodes: Vec<Node> = (0..=max_node_ind)
        .map(|_| Node::new())
        .collect();

    for edge in edges {
        let (src, dst) = (edge[0], edge[1]);
        nodes[src].next_nodes.push(dst);
        nodes[dst].prev_nodes.push(src);
    }

    nodes
}

fn parse_puzzle_input<R>(input: R) -> Vec<Vec<usize>>
    where R: io::BufRead
{
        input.lines()
            .skip(1)
            .map( |l| l.unwrap() )
            .map( |l| l.split(' ').map( |n| n.parse::<usize>().unwrap() ).collect() )
            .collect()
}

#[test]
fn test_parse_input() {
    let mut input_stream = io::Cursor::new(include_str!("../test_input_2.txt"));
    let parsed_output = parse_puzzle_input(&mut input_stream);
    assert_eq!(parsed_output, vec![]);
}

#[test]
fn test_nodes_from_edges() {
    let nodes = nodes_from_edges(&vec![
        vec![0, 3],
        vec![0, 2]
    ]);

    assert_eq!(nodes.len(), 4);
    
    assert_eq!(nodes[0].next_nodes, vec![3,2]);
    assert_eq!(nodes[0].prev_nodes, vec![]);

    assert_eq!(nodes[1].next_nodes, vec![]);
    assert_eq!(nodes[1].prev_nodes, vec![]);

    assert_eq!(nodes[2].next_nodes, vec![]);
    assert_eq!(nodes[2].prev_nodes, vec![0]);
    
    assert_eq!(nodes[3].next_nodes, vec![]);
    assert_eq!(nodes[3].prev_nodes, vec![0]);
}