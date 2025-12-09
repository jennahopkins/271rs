use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sha2::{Sha256, Digest};
use chrono::Local;

use std::fs;
use std::io::Write;
use std::env;

// sha256 of a string
fn sha256_string(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

// read lines from a file into a vector
fn read_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

// merkle root of a set of files (filename, lines)
fn merkle_root(files: &[(String, Vec<String>)]) -> String {
    let mut level: Vec<String> = files.iter()
        .map(|(_, lines)| sha256_string(&lines.join("\n")))
        .collect();

    if level.is_empty() {
        return sha256_string("EMPTY");
    }

    while level.len() > 1 {
        let mut next = Vec::new();

        for chunk in level.chunks(2) {
            if chunk.len() == 1 {
                next.push(sha256_string(&chunk[0]));
            } else {
                next.push(sha256_string(&(chunk[0].clone() + &chunk[1])));
            }
        }

        level = next;
    }

    level[0].clone()
}

// compute diff between two versions of a file
fn compute_diff(old: &[String], new: &[String]) -> Value {
    let deleted: Vec<String> = old.iter()
        .filter(|l| !new.contains(l))
        .cloned()
        .collect();

    let added: Vec<String> = new.iter()
        .filter(|l| !old.contains(l))
        .cloned()
        .collect();

    json!({ "deleted": deleted, "added": added })
}

// apply a diff to a version of a file
fn apply_diff(mut old: Vec<String>, diff: &Value) -> Vec<String> {
    let deletes = diff["deleted"].as_array().unwrap();
    let adds = diff["added"].as_array().unwrap();

    let dstrings: Vec<String> = deletes.iter().map(|v| v.as_str().unwrap().to_string()).collect();
    old.retain(|l| !dstrings.contains(l));

    for a in adds {
        old.push(a.as_str().unwrap().to_string());
    }

    old
}

// commit structure
#[derive(Serialize, Deserialize, Clone)]
struct Commit {
    timestamp: String,
    file: String,
    prev_hash: String,
    merkle_root: String,
    diff: Value,
    commit_hash: String,
}

// repository structure
#[derive(Serialize, Deserialize)]
struct Repository {
    tracked_file: String,
    init: Vec<String>,
    commits: Vec<Commit>,
}

// save repository to .scm file
fn save_repo(repo: &Repository) {
    fs::write(".scm", serde_json::to_string_pretty(repo).unwrap()).unwrap();
}

// load repository from .scm file
fn load_repo() -> Repository {
    let raw = fs::read_to_string(".scm").expect("No .scm found.");
    serde_json::from_str(&raw).unwrap()
}

// verify integrity of the repository
fn verify_repo(repo: &Repository) {
    let mut current = repo.init.clone();
    for c in &repo.commits {
        let expected_prev = sha256_string(&current.join("\n"));
        if expected_prev != c.prev_hash {
            panic!("SCM integrity failure: previous hash mismatch!");
        }
        current = apply_diff(current, &c.diff);

        let files = vec![(repo.tracked_file.clone(), current.clone())];
        let expected_root = merkle_root(&files);
        if expected_root != c.merkle_root {
            panic!("SCM integrity failure: Merkle root mismatch!");
        }

        let verify_hash = sha256_string(&format!("{}{}{}{}", c.timestamp, c.file, c.prev_hash, c.merkle_root));
        if verify_hash != c.commit_hash {
            panic!("SCM integrity failure: commit hash mismatch!");
        }
    }
}

// handle commit command
fn cmd_commit() {
    // Check if repo exists
    let mut repo = if fs::metadata(".scm").is_ok() {
        let mut r = load_repo();
        verify_repo(&r);
        r
    } else {
        // Auto-init
        let tracked_file = "file.txt";
        let init = read_lines(tracked_file);
        println!("Auto-initialized SCM tracking {}", tracked_file);
        Repository { tracked_file: tracked_file.to_string(), init, commits: vec![] }
    };

    let mut current = repo.init.clone();
    for c in &repo.commits {
        current = apply_diff(current, &c.diff);
    }

    let new_text = read_lines(&repo.tracked_file);
    let diff = compute_diff(&current, &new_text);
    let prev_hash = sha256_string(&current.join("\n"));
    let root = merkle_root(&[(repo.tracked_file.clone(), new_text.clone())]);
    let timestamp = Local::now().to_rfc3339();
    let commit_hash = sha256_string(&format!("{}{}{}{}", timestamp, &repo.tracked_file, prev_hash, root));

    let commit = Commit {
        timestamp,
        file: repo.tracked_file.clone(),
        prev_hash,
        merkle_root: root,
        diff,
        commit_hash,
    };

    repo.commits.push(commit);
    save_repo(&repo);
    println!("Committed {}", &repo.tracked_file);
}

// handle revert command
fn cmd_revert() {
    let repo = load_repo();
    verify_repo(&repo);

    if repo.commits.is_empty() {
        println!("No commits to revert to.");
        return;
    }

    // revert to the previous commit
    let mut text = repo.init.clone();
    for i in 0..repo.commits.len() - 1 {
        text = apply_diff(text, &repo.commits[i].diff);
    }

    let mut f = fs::File::create(&repo.tracked_file).unwrap();
    for line in text {
        writeln!(f, "{}", line).unwrap();
    }

    println!("Reverted {}", &repo.tracked_file);
}





fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: scm <commit|revert>");
        return;
    }

    match args[1].as_str() {
        "commit" => cmd_commit(),
        "revert" => cmd_revert(),
        _ => println!("Invalid SCM command."),
    }
}