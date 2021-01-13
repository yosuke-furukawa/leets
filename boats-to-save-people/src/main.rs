struct Solution{}

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let mut result = 0;
        let mut e: i32 = people.len() as i32 - 1;
        while people.len() > 0 {
            let start = people[0];
            let end = people[e as usize];
            if limit - start < start {
                result += 1;
                people.remove(0);
                e -= 1;
                continue;
            }
            if e == 0 {
                result += 1;
                people.remove(0);
                e = people.len() as i32 - 1;
                continue;
            }
            if limit - start - end >= 0 {
                result += 1;
                people.remove(e as usize);
                people.remove(0);
                e = people.len() as i32 - 1;
            } else if e > 0{
                e -= 1;
            } else {
                result += 1;
                people.remove(0);
                e = people.len() as i32 - 1;
            }
        }
        result
    }
}

fn main() {
    println!("{}", Solution::num_rescue_boats(vec![1,2], 3));
    println!("{}", Solution::num_rescue_boats(vec![3,2,2,1], 3));
    println!("{}", Solution::num_rescue_boats(vec![3,5,3,4], 5));
    println!("{}", Solution::num_rescue_boats(vec![2,4], 5));
    println!("{}", Solution::num_rescue_boats(vec![5,1,4,2], 6));
}
