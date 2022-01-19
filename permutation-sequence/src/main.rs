impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let array = (1..=n).collect::<Vec<i32>>();
        fn permutation(array: Vec<i32>, k: i32, results: &mut Vec<Vec<i32>>, candidate: Vec<i32>) {
            if array.is_empty() {
                results.push(candidate);
                return;
            }
            for i in 0..array.len() {
                let mut new_array = array.clone();
                let mut new_candidate = candidate.clone();
                new_candidate.push(array[i]);
                new_array.remove(i);
                permutation(new_array, k, results, new_candidate);
                if k == results.len() as i32 {
                    break;
                }
            }
        }
        let mut results = vec![];
        permutation(array, k, &mut results, vec![]);
        results[k as usize - 1]
            .iter()
            .map(|&x| x.to_string())
            .collect::<String>()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::get_permutation(3, 3));
    println!("{}", Solution::get_permutation(4, 9));
    println!("{}", Solution::get_permutation(3, 1));
}
