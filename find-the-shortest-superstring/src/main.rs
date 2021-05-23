impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let n = words.len();
        let mut overlaps = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let m = words[i].len().min(words[j].len());
                    for k in (0..m).rev() {
                        if words[i].ends_with(words[j].get(0..k).unwrap()) {
                            overlaps[i][j] = k;
                            break;
                        }
                    }
                }
            }
        }

        let mut dp = vec![vec![0; n]; 1<<n];
        let mut parent: Vec<Vec<i32>> = vec![vec![0; n]; 1<<n];

        for mask in 0..1<<n {
            parent[mask] = vec![-1; n];

            for bit in 0..n {
                if ((mask >> bit) & 1) > 0 {
                    let pmask = mask ^ (1 << bit);
                    if pmask == 0 {
                        continue;
                    }
                    for i in 0..n {
                        if (pmask >> i) & 1 > 0 {
                            let val = dp[pmask][i] + overlaps[i][bit];
                            if val > dp[mask][bit] {
                                dp[mask][bit] = val;
                                parent[mask][bit] = i as i32;
                            }
                        }
                    }
                }
            }
        }

        let mut perm = vec![0; n];
        let mut seen = vec![false; n];
        let mut t = 0;
        let mut mask = (1 << n) - 1;

        let mut p: i32 = 0;
        for j in 0..n {
            if dp[(1<<n) - 1][j] > dp[(1<<n) - 1][p as usize] {
                p = j as i32;
            }
        }

        // Follow parents down backwards path that retains maximum overlap
        while p != -1 {
            perm[t] = p;
            t += 1;
            seen[p as usize] = true;
            let p2 = parent[mask][p as usize];
            mask ^= 1 << p;
            p = p2;
        }

        // Reverse perm
        for i in 0..t/2 {
            let v = perm[i];
            perm[i] = perm[t-1-i];
            perm[t-1-i] = v;
        }

        // Fill in remaining words not yet added
        for i in 0..n {
            if !seen[i] {
                perm[t] = i as i32;
                t += 1;
            }
        }

        let mut ans = words[perm[0] as usize].clone();
        for i in 1..n {
            let overlap = overlaps[perm[i-1] as usize][perm[i] as usize];
            ans += words[perm[i] as usize].get(overlap..).unwrap();
        }
        ans
    }
}

struct Solution;

fn stringify(words: Vec<&str>) -> Vec<String> {
    words.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!("{}", Solution::shortest_superstring(stringify(vec!["catg","ctaagt","gcta","ttca","atgcatc"])));
    println!("{}", Solution::shortest_superstring(stringify(vec!["alex","loves","leetcode"])));
}
