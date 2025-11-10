use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    data: String,
    next: Vec<String>,
}

type Graph = HashMap<String, Node>;

fn main() {
    let stdin = io::stdin();
    let mut graph: Graph = HashMap::new();

    // read edges until an empty line
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<&str> = line.trim().split(':').collect();
        if parts.len() != 2 {
            eprintln!("Invalid line: {}", line);
            continue;
        }
        // end with prereq and course from each line
        let prereq = parts[0].to_string();
        let course = parts[1].to_string();

        // add the edge to the graph
        graph
            .entry(prereq.clone())
            .or_insert_with(|| Node {
                data: prereq.clone(),
                next: Vec::new(),
            })
            .next
            .push(course.clone());

        // ensure course node exists
        graph.entry(course.clone()).or_insert_with(|| Node {
            data: course.clone(),
            next: Vec::new(),
        });
    }

    dbg!(depth("C152", &graph));
    dbg!(depth("C371", &graph));
    dbg!(height("C152", &graph));
    dbg!(height("C371", &graph));
}


fn invert_graph(graph: &Graph) -> HashMap<String, Vec<String>> {
    let mut prereqs: HashMap<String, Vec<String>> = HashMap::new();
    for (k, node) in graph {
        // for each course, add its prerequisites
        for next in &node.next {
            prereqs.entry(next.clone()).or_default().push(k.clone());
        }
    }
    prereqs
}

fn depth(node: &str, graph: &Graph) -> usize {
    let prereqs = invert_graph(graph);
    // recursion with memoization
    fn helper(n: &str, prereqs: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, usize>) -> usize {
        // depth has already been computed
        if let Some(&val) = memo.get(n) {
            return val;
        }
        // hasn't been computed yet
        let d = match prereqs.get(n) {
            Some(parents) => 1 + parents.iter().map(|p| helper(p, prereqs, memo)).max().unwrap_or(0),
            None => 0,
        };
        // add to memo
        memo.insert(n.to_string(), d);
        d
    }
    helper(node, &prereqs, &mut HashMap::new())
}

fn height(node: &str, graph: &Graph) -> usize {
    // recursion with memoization
    fn helper(n: &str, graph: &Graph, memo: &mut HashMap<String, usize>) -> usize {
        // height has already been computed
        if let Some(&val) = memo.get(n) {
            return val;
        }
        // hasn't been computed yet
        let d = match graph.get(n) {
            Some(node) => node.next
                .iter()
                .map(|child| 1 + helper(child, graph, memo))
                .max()
                .unwrap_or(0),
            None => 0,
        };
        // add to memo
        memo.insert(n.to_string(), d);
        d
    }
    helper(node, graph, &mut HashMap::new())
}

