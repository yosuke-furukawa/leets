impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal_cuts = horizontal_cuts;
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        horizontal_cuts.sort_unstable();
        let mut vertical_cuts = vertical_cuts;
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        vertical_cuts.sort_unstable();
        let mut hmax = 0;
        for h in horizontal_cuts.windows(2) {
            hmax = hmax.max(h[1] - h[0]);
        }

        let mut vmax = 0;
        for v in vertical_cuts.windows(2) {
            vmax = vmax.max(v[1] - v[0]);
        }
        ((hmax as i64) * (vmax as i64) % 1000_000_007) as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]));
    println!("{}", Solution::max_area(5, 4, vec![3, 1], vec![1]));
    println!("{}", Solution::max_area(5, 4, vec![3], vec![3]));
}
