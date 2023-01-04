use std::collections::HashMap;

impl Solution {
    fn find_and_put(cache: &mut HashMap<i32, i32>, num: i32) -> i32 {
        if num < 0 {
            return -1;
        }
        if let Some(v) = cache.get(&num) {
            return *v;
        }
        let v = match num % 3 {
            0 => num / 3,
            2 => num / 3 + 1,
            1 => {
                let ans = Self::find_and_put(cache, num - 2);
                if ans < 0 {
                    -1
                } else {
                    ans + 1
                }
            }
            _ => unreachable!(),
        };
        cache.insert(num, v);
        v
    }
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut cache = HashMap::new();
        for task in tasks {
            *map.entry(task).or_insert(0) += 1;
        }
        let mut res = 0;
        for (_, v) in map {
            let ans = Self::find_and_put(&mut cache, v);
            if ans < 0 {
                return -1;
            }
            res += ans;
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4])
    );
    println!("{}", Solution::minimum_rounds(vec![2, 3, 3]));
}
