impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by_key(|a| (-a[0], a[1]));
        let mut res = vec![];
        for person in people {
            res.insert(person[1] as usize, person.to_vec());
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2]
        ])
    );
}
