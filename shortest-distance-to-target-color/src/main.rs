use std::collections::HashSet;

impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let colors_len = colors.len();
        let colset: HashSet<i32> = colors.clone().into_iter().collect();
        let mut colors_dict = vec![vec![1_000_000_000; 3]; colors_len];
        let mut answer = vec![];
        for (i, c) in colors.clone().into_iter().enumerate() {
            colors_dict[i][c as usize - 1] = 0;
            if i > 0 {
                for (j, n) in colors_dict[i - 1].clone().iter().enumerate() {
                    if c as usize - 1 != j && *n < 1_000_000_000 {
                        colors_dict[i][j] = n + 1;
                    }
                }
            }

            for (j, d) in colors_dict[i].clone().iter().enumerate() {
                if *d <= 0 {
                    continue;
                }
                let mut distance: i32 = 0;
                let mut found = false;
                for k in i..colors_len {
                    if colors[k] == j as i32 + 1 {
                        found = true;
                        break;
                    }
                    distance += 1;
                }
                if found {
                    colors_dict[i][j] = (*d).min(distance);
                }
            }
        }
        // println!("{:?}", colors_dict);

        for query in queries {
            if !colset.contains(&query[1]) {
                answer.push(-1);
                continue;
            }
            answer.push(colors_dict[query[0] as usize][query[1] as usize - 1]);
        }
        answer
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::shortest_distance_color(
            vec![1, 1, 2, 1, 3, 2, 2, 3, 3],
            vec![vec![1, 3], vec![2, 2], vec![6, 1]]
        )
    );
    println!(
        "{:?}",
        Solution::shortest_distance_color(vec![1, 2], vec![vec![0, 3]])
    );
    println!(
        "{:?}",
        Solution::shortest_distance_color(
            vec![2, 1, 2, 2, 1],
            vec![vec![1, 1], vec![4, 3], vec![1, 3], vec![4, 2], vec![2, 1]]
        )
    );
    println!(
        "{:?}",
        Solution::shortest_distance_color(
            vec![3, 2, 2, 1, 3, 1, 1, 1, 3, 1],
            vec![
                vec![4, 1],
                vec![9, 2],
                vec![4, 2],
                vec![8, 1],
                vec![0, 3],
                vec![2, 1],
                vec![2, 3],
                vec![6, 3],
                vec![4, 1],
                vec![1, 2]
            ]
        )
    );
}
