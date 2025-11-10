use std::collections::HashMap;
use std::io::{self, BufRead};


#[derive(Debug)]
struct Node {
    data: String,
    next: Vec<String>,
}

fn main() {
    let stdin = io::stdin();
    let mut graph: HashMap<String, Node> = HashMap::new();

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

        let prereq = parts[0].to_string();
        let course = parts[1].to_string();

        graph
            .entry(prereq.clone())
            .or_insert_with(|| Node {
                data: prereq.clone(),
                next: Vec::new(),
            })
            .next
            .push(course.clone());

        graph.entry(course.clone()).or_insert_with(|| Node {
            data: course.clone(),
            next: Vec::new(),
        });
    }

    let mut requires: HashMap<String, Vec<String>> = HashMap::new();

    for (k, node) in &graph {
        for next in &node.next {
            requires.entry(next.clone()).or_default().push(k.clone());
        }
    }

    for (course, prereqs) in requires {
        println!(
            "{} requires {}",
            course,
            prereqs.join(",")
        );
    }
}