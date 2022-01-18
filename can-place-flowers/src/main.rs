impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowers = vec![0];
        flowers.append(&mut flowerbed);
        flowers.push(0);
        let mut rest_flower = n;
        let mut skip_next = false;
        for f in flowers.windows(3) {
            if skip_next {
                skip_next = false;
                continue;
            }
            if f[0] == 0 && f[0] == f[1] && f[1] == f[2] {
                rest_flower -= 1;
                skip_next = true;
            }
            if rest_flower <= 0 {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    println!("{}", Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
}
