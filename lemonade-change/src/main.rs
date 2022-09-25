impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;
        for bill in bills {
            if bill == 5 {
                five += 1;
            } else if bill == 10 {
                if five == 0 {
                    return false;
                }
                five -= 1;
                ten += 1;
            } else {
                if ten > 0 && five > 0 {
                    ten -= 1;
                    five -= 1;
                } else if five >= 3 {
                    five -= 3;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    println!("{}", Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
}
