impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut x = 0;
        let mut y = 0;
        let mut i = 1;
        // 0: east, 1: south, 2: west, 3: north
        let mut direction = 0;
        while i <= n.pow(2) {
            matrix[y][x] = i;
            match (direction, x, y) {
                (0, _x, _y) if _x + 1 < n as usize && matrix[_y][_x + 1] == 0 => x += 1,
                (1, _x, _y) if _y + 1 < n as usize && matrix[_y + 1][_x] == 0 => y += 1,
                (2, _x, _y) if _x > 0 && matrix[_y][_x - 1] == 0 => x -= 1,
                (3, _x, _y) if _y > 0 && matrix[_y - 1][_x] == 0 => y -= 1,
                _ => {
                    direction = (direction + 1) % 4;
                    if direction == 0 {
                        x += 1;
                    } else if direction == 1 {
                        y += 1;
                    } else if direction == 2 {
                        x -= 1;
                    } else if direction == 3 {
                        y -= 1;
                    }
                }
            };
            i += 1;
        }
        matrix
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::generate_matrix(3));
}
