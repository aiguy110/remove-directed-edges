use std::io;

struct Node {
    prev_nodes: Vec<usize>,
    next_nodes: Vec<usize>,
    cached_max_trimmed_depth: Option<usize>
}

impl Node {
    fn new() -> Node {
        Node { 
            prev_nodes: vec![], 
            next_nodes: vec![],
            cached_max_trimmed_depth: None
        }
    }
}

fn main() {
    let edges  = parse_input_into_edges(io::BufReader::new(io::stdin()));
    let mut nodes  = nodes_from_edges(&edges);
    let answer = get_puzzle_answer(&mut nodes);
    println!("{}", answer);
}

fn get_puzzle_answer(nodes: &mut Vec<Node>) -> usize {
    (unsafe {&mut *(nodes as *mut Vec<Node>)}).iter()
        .enumerate() 
        .map(|(i, _)| max_trimmed_depth(unsafe {&mut *(nodes as *mut Vec<Node>)}, i))
        .max()
        .unwrap()
}

fn max_trimmed_depth(nodes: &mut Vec<Node>, n: usize) -> usize { 
    if let Some(depth) = nodes[n].cached_max_trimmed_depth {
        return depth;
    }
    
    if nodes[n].prev_nodes.len() <= 1 {
        return 1;
    }

    let sneeky_nodes_ptr = nodes as *mut Vec<Node>;
    let depth = (unsafe {&mut *sneeky_nodes_ptr})[n].prev_nodes.iter()
        .map(|&i| (i, &nodes[i]))
        .filter(|(_, prev_node)| prev_node.next_nodes.len() > 1)
        .map(|(i, _)| max_trimmed_depth(unsafe {&mut *sneeky_nodes_ptr}, i))
        .chain([0])
        .max()
        .unwrap() + 1;
    
    nodes[n].cached_max_trimmed_depth = Some(depth);
    
    depth
}

fn nodes_from_edges(edges: &Vec<Vec<usize>>) -> Vec<Node> {
    let &max_node_ind = edges.iter()
        .flatten()
        .chain([&0])
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

fn parse_input_into_edges<R>(input: R) -> Vec<Vec<usize>>
    where R: io::BufRead
{
        input.lines()
            .skip(1)
            .map( |l| l.unwrap() )
            .map( |l| l.split(' ').map( |n| n.parse::<usize>().unwrap() ).collect() )
            .collect()
}


#[test]
fn test_parse_input_1() {
    let mut input_stream = io::Cursor::new(include_str!("../test_input_1.txt"));
    let parsed_edges = parse_input_into_edges(&mut input_stream);
    let correct_edges = vec![
        vec![1,2],
        vec![2,3],
        vec![1,3]
    ];
    assert_eq!(parsed_edges, correct_edges);
}

#[test]
fn test_parse_input_2() {
    let mut input_stream = io::Cursor::new(include_str!("../test_input_2.txt"));
    let parsed_edges = parse_input_into_edges(&mut input_stream);
    let correct_edges: Vec<Vec<usize>> = Vec::new();
    assert_eq!(parsed_edges, correct_edges);
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

#[test]
fn test_puzzle_solution_1() {
    let mut input_stream = io::Cursor::new(include_str!("../test_input_1.txt"));
    let edges = parse_input_into_edges(&mut input_stream);
    let mut nodes = nodes_from_edges(&edges);
    assert_eq!(get_puzzle_answer(&mut nodes), 2);
}

#[test]
fn test_puzzle_solution_2() {
    let mut input_stream = io::Cursor::new(include_str!("../test_input_2.txt"));
    let edges = parse_input_into_edges(&mut input_stream);
    let mut nodes = nodes_from_edges(&edges);
    assert_eq!(get_puzzle_answer(&mut nodes), 1);
}

#[test]
fn test_puzzle_solution_3() {
    let mut input_stream = io::Cursor::new(include_str!("../test_input_3.txt"));
    let edges = parse_input_into_edges(&mut input_stream);
    let mut nodes = nodes_from_edges(&edges);
    assert_eq!(get_puzzle_answer(&mut nodes), 3);
}