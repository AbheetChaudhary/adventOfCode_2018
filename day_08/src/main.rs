type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input_file>", &args[0]);
        std::process::exit(1);
    }

    let untrimmed = std::fs::read_to_string(&args[1]).unwrap();
    let data_str = untrimmed.trim();

    let mut data_array = Vec::new();

    for token in data_str.split(' ') {
        data_array.push(token.parse::<i32>().unwrap());
    }

    part1(&data_array).unwrap();
    part2(&data_array).unwrap();

    Ok(())
}

type Stack = Vec<Node>;

struct Node {
    child_count: i32,    // how many childs this node has
    metadata_count: i32, // how much metadata this node has
    remaining: i32,      // how many childs are still remaining to process
}

impl Node {
    fn next_node(data_array: &Vec<i32>, idx: &mut usize) -> Self {
        let child_count = data_array[*idx];
        *idx += 1;
        let metadata_count = data_array[*idx];
        *idx += 1;

        Node {
            child_count,
            metadata_count,
            remaining: child_count,
        }
    }
}

fn part1(data_array: &Vec<i32>) -> Result<()> {
    let mut stack: Stack = Vec::new();
    let mut idx = 0; // to keep track of stack index
    let mut meta_sum = 0; // metadata sum
    stack.push(Node::next_node(&data_array, &mut idx));
    loop {
        if idx == data_array.len() {
            break;
        }

        let last = stack.last_mut().unwrap();

        if last.child_count == 0 {
            // no child nodes means what follows is metadata
            // read the metadata, pop the stack, and decrement the remaining count from the
            // parent(if any)
            meta_sum += get_metadata_sum(data_array, &mut idx, last.metadata_count);
            stack.pop();
            stack.last_mut().map(|node| node.remaining -= 1);
        } else if last.remaining == 0 {
            // all childs are done. What follows is current node's metadata. Read it then, pop the
            // stack and decrement the remaining child counter from the parent if one exist.
            meta_sum += get_metadata_sum(data_array, &mut idx, last.metadata_count);
            stack.pop();
            stack.last_mut().map(|node| node.remaining -= 1);
        } else {
            // last node has child nodes and not all of them have been processed.
            // So just push its next child node on the stack
            stack.push(Node::next_node(&data_array, &mut idx));
        }
    }
    println!("metadata sum: {}", meta_sum);

    Ok(())
}

#[derive(Debug)]
struct VerboseNode {
    child_count: i32,
    metadata: Vec<i32>, // using Option<Vec<i32>> for this vec will be to verbose
    weight: i32,
    value: i32,
}

impl VerboseNode {
    fn next_verbose_node(data_array: &Vec<i32>, mut idx: usize) -> Self {
        let child_count = data_array[idx];
        idx += 1;
        let metadata_count = data_array[idx] as usize;

        VerboseNode {
            child_count,
            metadata: Vec::with_capacity(metadata_count),
            weight: 0,
            value: 0,
        }
    }
}

fn part2(data_array: &Vec<i32>) -> Result<()> {
    let mut stack: Stack = Vec::new();
    let mut stack_verbose = Vec::new();
    let mut idx = 0; // to keep track of stack index
    let mut node_idx: i32 = -1;
    let mut node_idx_hist = Vec::new();
    stack_verbose.push(VerboseNode::next_verbose_node(&data_array, idx));
    stack.push(Node::next_node(&data_array, &mut idx));
    node_idx_hist.push(node_idx);
    node_idx = stack_verbose.len() as i32 - 1;
    loop {
        if idx == data_array.len() {
            break;
        }

        let last = stack.last_mut().unwrap();
        let last_verbose = stack_verbose.get_mut(node_idx as usize).unwrap();

        if last.child_count == 0 {
            // no child nodes means what follows is metadata
            // read the metadata, pop the stack, and decrement the remaining count from the
            // parent(if any)
            read_metadata(
                &data_array,
                &mut idx,
                last.metadata_count,
                &mut last_verbose.metadata,
            );
            stack.pop();
            node_idx = node_idx_hist.pop().unwrap();
            stack.last_mut().map(|node| node.remaining -= 1);
        } else if last.remaining == 0 {
            // all childs are done. What follows is current node's metadata. Read it then, pop the
            // stack and decrement the remaining child counter from the parent if one exist.
            read_metadata(
                &data_array,
                &mut idx,
                last.metadata_count,
                &mut last_verbose.metadata,
            );
            stack.pop();
            node_idx = node_idx_hist.pop().unwrap();
            stack.last_mut().map(|node| node.remaining -= 1);
        } else {
            // last node has child nodes and not all of them have been processed.
            // So just push its next child node on the stack
            stack_verbose.push(VerboseNode::next_verbose_node(&data_array, idx));
            stack.push(Node::next_node(&data_array, &mut idx));
            node_idx_hist.push(node_idx);
            node_idx = stack_verbose.len() as i32 - 1;
        }
    }
    complete_verbose_stack(&mut stack_verbose);

    println!("root value: {}", stack_verbose[0].value);

    Ok(())
}

fn read_metadata(data_array: &Vec<i32>, idx: &mut usize, len: i32, dst: &mut Vec<i32>) {
    for _ in 0..len {
        dst.push(data_array[*idx]);
        *idx += 1;
    }
}

fn get_metadata_sum(data_array: &Vec<i32>, idx: &mut usize, len: i32) -> i32 {
    let mut sum = 0;
    for _ in 0..len {
        sum += data_array[*idx];
        *idx += 1;
    }

    return sum;
}

fn complete_verbose_stack(stack: &mut Vec<VerboseNode>) {
    for idx in (0..stack.len()).rev() {
        stack[idx].weight = get_weight(stack, idx);
        stack[idx].value = get_value(stack, idx);
    }
}

fn get_value(stack: &Vec<VerboseNode>, idx: usize) -> i32 {
    let mut value: i32 = 0;
    if stack[idx].child_count == 0 {
        value += stack[idx].metadata.iter().sum::<i32>();
    } else {
        for &child_number in &stack[idx].metadata {
            if child_number > stack[idx].child_count {
                continue;
            }
            let mut child_idx = idx + 1;
            for _ in 1..child_number {
                child_idx += stack[child_idx].weight as usize;
            }
            value += stack[child_idx].value;
        }
    }

    value
}

fn get_weight(stack: &Vec<VerboseNode>, idx: usize) -> i32 {
    let mut weight = 1;
    if stack[idx].child_count != 0 {
        let mut child_idx = idx as usize + 1;
        for _ in 0..stack[idx].child_count {
            weight += stack[child_idx].weight;
            child_idx += stack[child_idx].weight as usize;
        }
    }

    weight
}
