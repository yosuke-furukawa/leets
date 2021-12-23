impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut course_map: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
        let mut indegrees = vec![0; num_courses as usize];
        let mut queue: Vec<i32> = vec![];
        let mut order: Vec<i32> = vec![];

        for edge in prerequisites {
            let req = edge[1];
            let course = edge[0];
            course_map[req as usize].push(course);
            indegrees[course as usize] += 1;
        }

        for (i, &indegree) in indegrees.iter().enumerate() {
            if indegree == 0 {
                queue.push(i as i32);
            }
        }

        while !queue.is_empty() {
            let course = queue.pop().unwrap();
            order.push(course);
            let reqs = &course_map[course as usize];
            for obj in reqs {
                indegrees[*obj as usize] -= 1;
                if indegrees[*obj as usize] == 0 {
                    queue.push(*obj);
                }
            }
        }

        if order.len() != num_courses as usize {
            let order: Vec<i32> = vec![];
            return order;
        }

        order
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
    println!("{:?}", Solution::find_order(2, grid![[1, 0]]));
}
