impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let paths: Vec<&str> = input.split('\n').collect();
        let mut stack = vec![0];
        let mut max = 0;
        for path in paths {
            let level = path.matches('\t').count();
            while level + 1 < stack.len() {
                stack.pop();
            }
            let len = stack.last().unwrap() + path.len() - level + 1;
            stack.push(len);
            if path.contains('.') {
                max = max.max(len - 1);
            }
        }
        max as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string())
    );
    println!("{}", Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
    .to_string()));
}
