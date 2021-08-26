impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let leaves: Vec<&str> = preorder.split(',').collect();
        let len = leaves.len();
        let mut stack = vec![];

        for (i, leaf) in leaves.into_iter().enumerate() {
            match leaf {
                "#" => {
                    if i != len - 1 && !stack.is_empty() {
                        stack.pop();
                    } else if i == len - 1 {
                        continue;
                    } else {
                        return false;
                    }
                }
                _ => stack.push(leaf),
            }
        }
        stack.is_empty()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string())
    );
    println!("{}", Solution::is_valid_serialization("1,#".to_string()));
    println!(
        "{}",
        Solution::is_valid_serialization("9,#,#,1".to_string())
    );
    println!(
        "{}",
        Solution::is_valid_serialization("#,7,6,9,#,#,#".to_string())
    );
}
