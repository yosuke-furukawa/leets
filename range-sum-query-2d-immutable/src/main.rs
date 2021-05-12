struct NumMatrix {
    ft: FenwickTree,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self {
            ft: FenwickTree::new(matrix),
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.ft
            .query2d(row1 as usize, col1 as usize, row2 as usize, col2 as usize)
    }
}

struct FenwickTree {
    matrix: Vec<Vec<i32>>,
}

impl FenwickTree {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() {
            return Self {
                matrix: vec![vec![]],
            };
        }
        let n = matrix.len();
        let m = matrix[0].len();
        let mut array = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                let value = matrix[i][j];
                FenwickTree::update(&mut array, (i + 1) as i32, (j + 1) as i32, value);
            }
        }

        FenwickTree { matrix: array }
    }

    fn lsb(x: i32) -> i32 {
        x & -x
    }

    fn query(&self, x: usize, y: usize) -> i32 {
        let mut sum = 0;
        let mut xi = x;
        while xi > 0 {
            let mut yi = y;
            while yi > 0 {
                sum += self.matrix[xi][yi];
                yi -= FenwickTree::lsb(yi as i32) as usize;
            }
            xi -= FenwickTree::lsb(xi as i32) as usize;
        }
        sum
    }

    fn query2d(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
        let res1 = self.query(x2 + 1, y2 + 1);
        let res2 = self.query(x1, y2 + 1);
        let res3 = self.query(x2 + 1, y1);
        let res4 = self.query(x1, y1);
        res1 - res2 - res3 + res4
    }

    fn update(array: &mut Vec<Vec<i32>>, x: i32, y: i32, value: i32) {
        let mut xi = x;
        while xi < array.len() as i32 {
            let mut yi = y;
            while yi < array[0].len() as i32 {
                array[xi as usize][yi as usize] += value;
                yi += FenwickTree::lsb(yi);
            }
            xi += FenwickTree::lsb(xi);
        }
    }
}

fn main() {
    let num_matrix = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);
    println!("{}", num_matrix.sum_region(2, 1, 4, 3));
    println!("{}", num_matrix.sum_region(1, 1, 2, 2));
    println!("{}", num_matrix.sum_region(1, 2, 2, 4));
    println!("{:?}", num_matrix.ft.matrix);
}
