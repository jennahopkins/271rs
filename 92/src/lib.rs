
// traits push and pop for generic Stack and Queue

pub trait Push<T> {
    fn push(self, val: T) -> Self;
}

pub trait Pop<T> {
    fn pop(self) -> (Option<T>, Self);
}


// generic Stack and Queue implementations

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Stack<T> {
    vals: Option<Node<T>>,
}

#[derive(Debug)]
struct QNode<T> {
    data: T,
    next: Option<Box<QNode<T>>>,
}

#[derive(Debug)]
pub struct Queue<T> {
    head: Option<QNode<T>>,
}

pub fn queue<T>() -> Queue<T> {
    return Queue {
        head: None,
    };
}

pub fn stack<T>() -> Stack<T> {
    return Stack {
        vals: None,
    };
}


// implementations of push and pop for Stack and Queue

impl<T> Push<T> for Stack<T> {
    fn push(mut self, val: T) -> Self {
        match self.vals.take() {
            None => {
                // empty stack: create first node
                self.vals = Some(Node { data: val, next: None });
            }
            Some(node) => {
                // not empty: create a new node pointing to the existing stack
                let new_node = Node {
                    data: val,
                    next: Some(Box::new(node)),
                };
                self.vals = Some(new_node);
            }
        }
        self
    }
}

impl<T> Pop<T> for Stack<T> {
    fn pop(mut self) -> (Option<T>, Self) {
        match self.vals.take() {
            None => (None, self),
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
}

impl<T> Push<T> for Queue<T> {
    fn push(mut self, val: T) -> Self {
        let new_node = QNode { data: val, next: None };

        match self.head.as_mut() {
            None => {
                // empty queue: first node becomes head
                self.head = Some(new_node);
            }
            Some(mut node) => {
                // non-empty: walk to the end (tail) and insert there
                while let Some(ref mut next) = node.next {
                    node = next;
                }
                node.next = Some(Box::new(new_node));
            }
        }
        self
    }
}

impl<T> Pop<T> for Queue<T> {
    fn pop(mut self) -> (Option<T>, Self) {
        match self.head.take() {
            None => (None, self), // empty queue
            Some(node) => {
                // pop from the head
                let QNode { data, next } = node;
                let new_head = next.map(|boxed| *boxed);
                let new_queue = Queue { head: new_head };
                (Some(data), new_queue)
            }
        }
    }
}
