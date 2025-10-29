#[derive(Debug)]
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Stack {
    vals: Option<Node>,
}

pub fn init() -> Stack {
    return Stack {
        vals: None,
    };
}

pub fn push(val: String, mut s: Stack) -> Stack {
    match s.vals.take() {
        None => {
            // empty stack and end of recursion: create first node
            s.vals = Some(Node { data: val, next: None });
        }
        Some(node) => {
            // not empty: create a new node pointing to the existing stack
            let new_node = Node {
                data: val,
                next: Some(Box::new(node)),
            };
            s.vals = Some(new_node);
        }
    }
    s
}

pub fn pop(mut s: Stack) -> (Option<String>, Stack) {
   match s.vals.take() {
        None => (None, s),
            // already empty stack
        Some(node) => {
            // non-empty stack: take the top node off
            let Node {data, next} = node;
            let new_vals = if next.is_none() {None} else {Some(*next.unwrap())};
            let new_stack = Stack {
                vals: new_vals,
            };
            (Some(data), new_stack)
        }
    }
}