impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let perim: i32 = matchsticks.iter().sum();
        perim % 4 == 0 && Self::solve(&mut matchsticks[..], 0, 0, 1, perim / 4)
    }

    fn solve(sticks: &mut [i32], l: usize, len: i32, side: u8, max: i32) -> bool {
        side == 4
            || len == max && Self::solve(sticks, 0, 0, side + 1, max)
            || (l..sticks.len()).any(|i| {
                let stick = std::mem::take(&mut sticks[i]);
                let res = stick != 0 && Self::solve(sticks, i + 1, len + stick, side, max);
                sticks[i] = stick;
                res
            })
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::makesquare(vec![1, 1, 2, 2, 2]));
    println!(
        "{}",
        Solution::makesquare(vec![1, 2, 2, 2, 3, 2, 1, 1, 1, 1, 4])
    );
    println!("{}", Solution::makesquare(vec![3, 3, 3, 3, 4]));
    println!(
        "{}",
        Solution::makesquare(vec![
            5969561, 8742425, 2513572, 3352059, 9084275, 2194427, 1017540, 2324577, 6810719,
            8936380, 7868365, 2755770, 9954463, 9912280, 4713511
        ])
    );
    println!(
        "{}",
        Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3])
    );
    println!(
        "{}",
        Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 102])
    );
}
