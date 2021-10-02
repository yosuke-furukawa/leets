impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.len() == 0 || dungeon[0].len() == 0 {
            return 1;
        }
        if (dungeon.len() == 1 || dungeon[0].len() == 1) && dungeon[0][0] >= 0 {
            return 1;
        }
        if (dungeon.len() == 1 || dungeon[0].len() == 1) && dungeon[0][0] < 0 {
            return dungeon[0][0] * -1 + 1;
        }
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut hp = vec![vec![0; n]; m];
        hp[m - 1][n - 1] = 1 - dungeon[m - 1][n - 1].min(0);
        for y in (0..=n - 2).rev() {
            hp[m - 1][y] = (hp[m - 1][y + 1] - dungeon[m - 1][y]).max(1);
        }
        for x in (0..=m - 2).rev() {
            hp[x][n - 1] = (hp[x + 1][n - 1] - dungeon[x][n - 1]).max(1);
        }

        for x in (0..=m - 2).rev() {
            for y in (0..=n - 2).rev() {
                let down = (hp[x + 1][y] - dungeon[x][y]).max(1);
                let right = (hp[x][y + 1] - dungeon[x][y]).max(1);
                hp[x][y] = down.min(right);
            }
        }
        hp[0][0]
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![
                $(
                    vec![$($x), *],
                )*
            ]
        }
    };
}

fn main() {
    println!(
        "{}",
        Solution::calculate_minimum_hp(grid![[-2, -3, 3], [-5, -10, 1], [10, 30, -5]])
    );
    println!(
        "{}",
        Solution::calculate_minimum_hp(grid![[0]])
    );
    println!(
        "{}",
        Solution::calculate_minimum_hp(grid![[-200]])
    );
}
