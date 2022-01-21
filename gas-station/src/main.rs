impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut answer = -1;
        let len = gas.len();
        if len == 1 && gas[0] >= cost[0] {
            return 0;
        }
        for i in 0..len {
            let mut tank = 0;
            let mut position = i;
            if gas[i] <= cost[i] {
                continue;
            }
            loop {
                tank += gas[position];
                tank -= cost[position];
                if tank < 0 {
                    break;
                }
                position = (position + 1) % len;
                if position == i {
                    answer = i as i32;
                    break;
                }
            }
        }
        answer
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    );
    println!(
        "{}",
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
    );
    println!(
        "{}",
        Solution::can_complete_circuit(vec![0, 0, 2], vec![0, 0, 1])
    );
}
