impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut uf = UnionFind::new(26);
        let equals = equations.iter().filter(|s| s.as_bytes()[1] == b'=');
        let not_equals = equations.iter().filter(|s| s.as_bytes()[1] == b'!');
        for equal in equals {
            let chars: Vec<char> = equal.chars().collect();
            let c1 = chars[0].min(chars[3]) as usize - 'a' as usize;
            let c2 = chars[0].max(chars[3]) as usize - 'a' as usize;

            if c1 == c2 && chars[1] == '=' {
                continue;
            }
            uf.union(c1, c2);
        }
        for not_equal in not_equals {
            let chars: Vec<char> = not_equal.chars().collect();
            let c1 = chars[0].min(chars[3]) as usize - 'a' as usize;
            let c2 = chars[0].max(chars[3]) as usize - 'a' as usize;
            if c1 == c2 && chars[1] == '!' {
                return false;
            }
            if uf.find(c1) == uf.find(c2) {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind { parent }
    }

    fn find(&self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            i
        } else {
            self.find(j)
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
        }
    }
}

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|&s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::equations_possible(stringify(vec!["a==b", "b!=a"]))
    );
    println!(
        "{}",
        Solution::equations_possible(stringify(vec!["a==b", "b==a"]))
    );
    println!(
        "{}",
        Solution::equations_possible(stringify(vec!["a==b", "b!=c", "c==a"]))
    );
    println!(
        "{}",
        Solution::equations_possible(stringify(vec!["c==c", "b==d", "x!=z"]))
    );
}
