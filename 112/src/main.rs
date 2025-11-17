fn lcs(s1: &str, s2: &str) -> String {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();

    let m = b.len(); // rows
    let n = a.len(); // columns

    // initialize dp table
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    // build dp table
    for i in 1..=m {
        for j in 1..=n {
            if b[i - 1] == a[j - 1] {
                // characters match = diagonal + 1
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                // differ = choose max of top or left
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // reconstruct
    let mut i = m;
    let mut j = n;
    let mut result = Vec::new();

    while i > 0 && j > 0 {
        if b[i - 1] == a[j - 1] {
            // match = move diagonally, collect
            result.push(b[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            // differ; up is greater
            i -= 1;
        } else {
            // differ; left is greater
            j -= 1;
        }
    }

    result.reverse();
    return result.iter().collect();
}

// has to run from terminal, doesn't work when used in debug vscode
fn main() {
    let mut ss = std::env::args();
    let _ = &ss.next();
    dbg!(lcs(&ss.next().unwrap(), &ss.next().unwrap()));
    return;
}