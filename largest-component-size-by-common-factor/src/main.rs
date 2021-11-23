use std::collections::HashMap;

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().unwrap();
        let mut dsu = DisjointSetUnion::new(*max as usize);
        let mut num_factor: HashMap<i32, i32> = HashMap::new();

        for n in nums.iter() {
            let pd = prime_decompose(*n);
            num_factor.insert(*n, pd[0]);
            for i in 0..pd.len() - 1 {
                dsu.union(pd[i], pd[i + 1]);
            }
        }

        let mut max_groups = 0;
        let mut group = HashMap::new();
        for n in nums.iter() {
            let group_id = dsu.find(*num_factor.get(n).unwrap());
            let v = group.entry(group_id).or_insert(0);
            *v += 1;
            max_groups = max_groups.max(*v);
        }
        max_groups
    }
}

fn prime_decompose(num: i32) -> Vec<i32> {
    let mut num = num;
    let mut prime_factors = vec![];
    let mut factor = 2;
    while num >= factor * factor {
        if num % factor == 0 {
            prime_factors.push(factor);
            num /= factor;
        } else {
            factor += 1;
        }
    }
    prime_factors.push(num);
    prime_factors
}

struct DisjointSetUnion {
    parent: Vec<i32>,
    size: Vec<i32>,
}

impl DisjointSetUnion {
    fn new(size: usize) -> DisjointSetUnion {
        let mut union = DisjointSetUnion {
            parent: vec![0; size + 1],
            size: vec![0; size + 1],
        };
        for i in 0..size + 1 {
            union.parent[i] = i as i32;
            union.size[i] = i as i32;
        }
        union
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] != x {
            self.parent[x as usize] = self.find(self.parent[x as usize]);
        }
        self.parent[x as usize]
    }

    fn union(&mut self, x: i32, y: i32) -> i32 {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py {
            return px;
        }

        if self.size[px as usize] > self.size[py as usize] {
            std::mem::swap(&mut py, &mut px);
        }

        self.parent[px as usize] = py;
        self.size[py as usize] += self.size[px as usize];
        py
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::largest_component_size(vec![4, 6, 15, 35]));
    println!("{}", Solution::largest_component_size(vec![20, 50, 9, 63]));
    println!(
        "{}",
        Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39])
    );
}
