impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut graph = vec![vec![]; n];

        for r in roads {
            let (u, v) = (r[0] as usize, r[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut flag = vec![0; n];
        let mut ret = 0;

        flag[0] = 1;
        Self::dfs(0, &graph, &mut flag, &mut ret, seats);

        ret
    }

    fn dfs(
        u: usize,
        graph: &Vec<Vec<usize>>,
        flag: &mut Vec<i32>,
        ret: &mut i64,
        seats: i32,
    ) -> i32 {
        let mut count = 1;
        for v in &graph[u] {
            if flag[*v] == 1 {
                continue;
            }

            flag[*v] = 1;
            let temp = Self::dfs(*v, graph, flag, ret, seats);
            *ret += ((temp + seats - 1) / seats) as i64;
            count += temp;
        }

        count
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::minimum_fuel_cost(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], 2)
    );
}
