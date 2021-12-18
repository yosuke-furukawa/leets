impl Solution {
    pub fn at_most_n_given_digit_set(d: Vec<String>, n: i32) -> i32 {
        let b = d.len();
        let s = n.to_string();
        let k = s.len();
        let mut a = vec![0; k];
        let mut t = 0;
        for &ch in s.as_bytes() {
            let mut c_index = 0;
            let mut is_match = false;

            for i in 0..b {
                if ch >= d[i].as_bytes()[0] {
                    c_index = i + 1;
                }
                if ch == d[i].as_bytes()[0] {
                    is_match = true;
                }
            }
            a[t] = c_index;
            t += 1;
            if is_match {
                continue;
            }

            if c_index == 0 {
                for j in (1..t).rev() {
                    if a[j] > 0 {
                        break;
                    }
                    a[j] += b;
                    a[j - 1] -= 1;
                }
            }

            while t < k {
                a[t] = b;
                t += 1;
            }
            break;
        }
        let mut ans = 0;
        for x in a {
            ans = ans * b + x;
        }
        ans as i32
    }
}

struct Solution;

fn stringify(arr: Vec<&str>) -> Vec<String> {
    arr.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::at_most_n_given_digit_set(stringify(vec!["1", "3", "5", "7"]), 100)
    );
}
