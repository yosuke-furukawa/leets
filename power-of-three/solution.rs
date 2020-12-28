impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
      return n > 0 && (3 as i32).pow(19) % n == 0;  
    }
}
