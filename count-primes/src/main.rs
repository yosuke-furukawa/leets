impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut is_primes = vec![true; n as usize];
        is_primes[0] = false;
        is_primes[1] = false;

        let mut i = 2_usize;
        while i * i < n as usize {
            if !is_primes[i] {
                i += 1;
                continue;
            }

            let mut j = i * i;
            while j < n as usize {
                is_primes[j] = false;
                j += i;
            }
            i += 1;
        }
        is_primes.into_iter().filter(|b| *b).count() as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_primes(104));
}
