struct Solution{}

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let mut result = 0;
        let mut s: i32 = 0;
        let mut e: i32 = people.len() as i32 - 1;
        while s <= e {
            result += 1;
            if people[s as usize] + people[e as usize] <= limit {
                s += 1;
            }
            e -= 1;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::num_rescue_boats(vec![1,2], 3));
    println!("{}", Solution::num_rescue_boats(vec![3,2,2,1], 3));
    println!("{}", Solution::num_rescue_boats(vec![3,5,3,4], 5));
    println!("{}", Solution::num_rescue_boats(vec![2,4], 5));
    println!("{}", Solution::num_rescue_boats(vec![5,1,4,2], 6));
}
