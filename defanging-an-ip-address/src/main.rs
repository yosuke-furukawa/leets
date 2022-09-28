impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.', "[.]")
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::defang_i_paddr("1.1.1.1".to_string()));
}
