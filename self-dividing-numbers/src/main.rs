impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];
        for n in left..=right {
            let nums = n.to_string().chars().collect::<Vec<char>>();
            let mut flag = true;
            for num in nums {
                let m = num as i32 - '0' as i32;
                if m == 0 {
                    flag = false;
                    break;
                }
                if n % m != 0 {
                    flag = false;
                    break;
                }
            }
            if flag {
                res.push(n);
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::self_dividing_numbers(1, 22));
}
