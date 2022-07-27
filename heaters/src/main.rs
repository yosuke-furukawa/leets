impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        houses.sort_unstable();
        let mut heaters = heaters;
        heaters.sort_unstable();
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        while i < houses.len() {
            while j < heaters.len() - 1
                && (houses[i] - heaters[j]).abs() >= (houses[i] - heaters[j + 1]).abs()
            {
                j += 1;
            }
            ans = ans.max((houses[i] - heaters[j]).abs());
            i += 1;
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_radius(vec![1, 2, 3], vec![2]));
    println!("{}", Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]));
    println!("{}", Solution::find_radius(vec![1, 5], vec![2]));
}
