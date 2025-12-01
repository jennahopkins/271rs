use std::fs;

// read lines from a file
fn fname_to_lines(fname: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in fs::read_to_string(fname).unwrap().lines() {
        lines.push(String::from(line));
    }
    return lines;
}

// lcs over lines
fn lcs_table(a: &Vec<String>, b: &Vec<String>) -> Vec<Vec<usize>> {
    let m = b.len();
    let n = a.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if b[i - 1] == a[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp
}

// backtrack through dp table to mark lcs matches
fn backtrack_lcs(
    a: &Vec<String>, 
    b: &Vec<String>, 
    dp: &Vec<Vec<usize>>
) -> Vec<(usize, usize)> {

    let mut i = b.len();
    let mut j = a.len();

    let mut matches = Vec::new();

    while i > 0 && j > 0 {
        if a[j - 1] == b[i - 1] {
            // match → part of LCS
            matches.push((i - 1, j - 1));
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    matches.reverse();
    matches
}

// convert lcs match list into diff format
fn build_diff(a: &Vec<String>, b: &Vec<String>, matches: Vec<(usize, usize)>) -> Vec<String> {
    let mut out = Vec::<String>::new();

    let mut ai = 0; // index in A
    let mut bi = 0; // index in B
    let mut mi = 0; // index in matches

    fn range(s: usize, e: usize) -> String {
        if s == e { format!("{}", s + 1) }
        else { format!("{},{}", s + 1, e + 1) }
    }

    while ai < a.len() || bi < b.len() {

        // if the next lines match, just consume both
        if mi < matches.len() && matches[mi].1 == ai && matches[mi].0 == bi {
            ai += 1;
            bi += 1;
            mi += 1;
            continue;
        }

        // otherwise, entering an unmatched region.
        let start_ai = ai;
        let start_bi = bi;

        let mut dels = Vec::<usize>::new();
        let mut adds = Vec::<usize>::new();

        // collect deletions until the next match on A
        while ai < a.len() && !(mi < matches.len() && matches[mi].1 == ai) {
            dels.push(ai);
            ai += 1;
        }

        // collect additions until the next match on B
        while bi < b.len() && !(mi < matches.len() && matches[mi].0 == bi) {
            adds.push(bi);
            bi += 1;
        }

        // classify hunk
        let htype = match (dels.is_empty(), adds.is_empty()) {
            (false, true) => "d",
            (true, false) => "a",
            (false, false) => "c",
            _ => unreachable!()
        };


        // left side range (file A)
        let (lstart, lend) = if dels.is_empty() {
            let pos = if start_ai == 0 { 0 } else { start_ai - 1 };
            (pos, pos)
        } else {
            (dels[0], *dels.last().unwrap())
        };

        // right side range (file B)
        let (rstart, rend) = if adds.is_empty() {
            let pos = if start_bi == 0 { 0 } else { start_bi - 1 };
            (pos, pos)
        } else {
            (adds[0], *adds.last().unwrap())
        };

        // header
        out.push(format!("{}{}{}",
            range(lstart, lend),
            htype,
            range(rstart, rend)
        ));

        // deletions
        for &d in &dels {
            out.push(format!("< {}", a[d]));
        }

        if htype == "c" {
            out.push("---".to_string());
        }

        // additions
        for &x in &adds {
            out.push(format!("> {}", b[x]));
        }
    }

    out
}


fn main() {
    let mut args = std::env::args();
    args.next(); // skip program name

    let f1 = args.next().unwrap();
    let f2 = args.next().unwrap();

    let a = fname_to_lines(&f1);
    let b = fname_to_lines(&f2);

    let dp = lcs_table(&a, &b);
    let matches = backtrack_lcs(&a, &b, &dp);

    let mut diff = build_diff(&a, &b, matches);



    for l in diff {
        println!("{}", l);
    }
}