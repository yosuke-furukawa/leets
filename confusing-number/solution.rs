impl Solution {
    pub fn confusing_number(n: i32) -> bool {
      let mut num:i32 = n;
      let mut result:i32 = 0;
      while num > 0 {
        let mut mod10 = num % 10;
        let numtype:bool = match mod10 {
          2 | 3 | 4 | 5 | 7 => false,
          _ => true,
        };
        
        if !numtype {
          return false;
        }
        
        if mod10 == 6 {
          mod10 = 9;
        } else if mod10 == 9 {
          mod10 = 6;
        }
        result = result * 10 + mod10;
        num /= 10;
      }
      return result != n;
    }
}
