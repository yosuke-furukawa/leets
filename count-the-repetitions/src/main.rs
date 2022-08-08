impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        if n1 == 0 {
            return 0;
        };
        let mut indices = vec![0; n1 as usize + 1];
        let mut counts = vec![0; n1 as usize + 1];
        let mut index = 0;
        let mut count = 0;
        for i in 1..=n1 as usize {
            for j in 0..s1.len() {
                if s1.as_bytes()[j] == s2.as_bytes()[index] {
                    index += 1;
                }
                if index == s2.len() {
                    index = 0;
                    count += 1;
                }
            }
            counts[i] = count;
            indices[i] = index;
            for k in 0..i {
                if indices[k] == index {
                    let pre_count = counts[k];
                    let pattern_count = (n1 - k as i32) / (i - k) as i32 * (counts[i] - pre_count);
                    let remain_count = counts[k + (n1 as usize - k) % (i - k)] - pre_count;
                    return (pre_count + pattern_count + remain_count) / n2;
                }
            }
        }
        counts[n1 as usize] / n2
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2)
    );
    println!(
        "{}",
        Solution::get_max_repetitions("baba".to_string(), 11, "baab".to_string(), 1)
    );
}
