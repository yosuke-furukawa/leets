impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        for i in 0..n {
            for j in 0..n {
                if i != j && self.knows(i, j) {
                    break;
                }
                if j == n - 1 {
                    let everyone_knows_i = (0..n).all(|k| {
                        if k == i {
                            true
                        } else {
                            self.knows(k, i)
                        }
                    });
                    if everyone_knows_i {
                        return i;
                    } else {
                        return -1;
                    }
                }
            }
        }
        return -1
    }
}

struct Solution;

impl Solution {
    #[warn(unused_variables)]
    fn knows(&self, a: i32, b: i32) -> bool {
        true
    }
}

fn main() {
    let s = Solution{};
    println!("{}", s.find_celebrity(3));
}
