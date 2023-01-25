impl Solution {
    fn helper(edges: &[i32], visited: &mut Vec<i32>, node: i32, depth: i32) {
        if node == -1 {
            return;
        }
        if visited[node as usize] >= 0 {
            return;
        }
        let next = edges[node as usize];
        visited[node as usize] = depth;
        Self::helper(edges, visited, next, depth + 1);
    }
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let len = edges.len() as i32;
        let mut visited1 = vec![-1; len as usize];
        let mut visited2 = vec![-1; len as usize];
        Self::helper(&edges, &mut visited1, node1, 0);
        Self::helper(&edges, &mut visited2, node2, 0);
        let mut min = std::i32::MAX;
        let mut ans = -1;
        for i in 0..len {
            if visited1[i as usize] != -1 && visited2[i as usize] != -1 {
                let m = visited1[i as usize].max(visited2[i as usize]);
                if m < min {
                    min = m;
                    ans = i;
                }
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1)
    );
    println!("{}", Solution::closest_meeting_node(vec![1, 2, -1], 0, 2));
    println!("{}", Solution::closest_meeting_node(vec![2, 0, 0], 2, 0));
    println!(
        "{}",
        Solution::closest_meeting_node(vec![4, 3, 0, 5, 3, -1], 4, 0)
    );
    println!(
        "{}",
        Solution::closest_meeting_node(vec![4, 4, 8, -1, 9, 8, 4, 4, 1, 1], 5, 6)
    );
}
