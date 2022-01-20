impl Solution {
    fn can_eat(piles: &[i32], mid: i32, h: i32) -> bool {
        let mut hours = 0;
        for pile in piles {
            let mut dh = *pile / mid;
            dh += if *pile % mid > 0 { 1 } else { 0 };
            hours += dh;
        }
        hours <= h
    }
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = 1000000000;
        while left < right {
            let mid = (left + right) / 2;
            if Solution::can_eat(&piles, mid, h) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
    println!("{}", Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5));
    println!("{}", Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6));
    println!(
        "{}",
        Solution::min_eating_speed(
            vec![
                332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673,
                679580077, 337406589, 290818316, 877337160, 901728858, 679284947, 688210097,
                692137887, 718203285, 629455728, 941802184
            ],
            823855818
        )
    );
}
