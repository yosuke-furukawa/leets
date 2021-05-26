impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars()
            .map(|d| d.to_string().parse::<i32>().unwrap())
            .fold(0, |max, n| max.max(n))
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_partitions("32".to_string()));
    println!("{}", Solution::min_partitions("21".to_string()));
    println!("{}", Solution::min_partitions("1".to_string()));
    println!("{}", Solution::min_partitions("2".to_string()));
    println!(
        "{}",
        Solution::min_partitions("432809482904890238490238904890234".to_string())
    );
}
