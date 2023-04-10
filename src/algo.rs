pub fn osa_dist(s1: &str, s2: &str) -> usize { 
    let mut dp = [vec![0; s1.len()+1], vec![0; s2.len()+1]];
    let b1: &[u8] = s1.as_bytes();
    let b2: &[u8] = s2.as_bytes();

    for i in 0..s1.len()+1 {
        dp[i][0] = i;
    }
    for j in 0..s2.len()+1 {
        dp[0][j] = j;
    }

    for i in 1..s1.len()+1 {
        for j in 1..s2.len()+1 {
            if &b1[i-1] == &b2[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + [dp[i-1][j], dp[i][j-1], dp[i-1][j-1]].iter().min().unwrap();
            }
        }
    }
    return dp[s1.len()][s2.len()]
}
