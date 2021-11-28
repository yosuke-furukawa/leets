impl Solution {
    fn helper(
        node: usize,
        target: usize,
        temp: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        graph: &[Vec<i32>],
    ) {
        let mut temp = temp;
        temp.push(node as i32);
        if node == target {
            result.push(temp.clone());
            return;
        }
        for i in 0..graph[node].len() {
            Self::helper(graph[node][i] as usize, target, temp.clone(), result, graph);
        }
    }
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::helper(0, graph.len() - 1, vec![], &mut result, &graph);
        result
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
        "{:?}",
        Solution::all_paths_source_target(grid![[1, 2], [3], [3], []])
    );
}
