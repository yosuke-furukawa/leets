impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut current = vec![];
        let result = "/".to_string();
        let paths: Vec<&str> = path.split('/').collect();
        for p in paths {
            match p {
                ".." => {
                    current.pop();
                }
                "." => {
                    continue;
                }
                path if !path.is_empty() => {
                    current.push(path);
                }
                _ => {
                    continue;
                }
            }
        }
        result + current.join("/").as_str()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::simplify_path("/foo/bar/../baz".to_string()));
    println!("{}", Solution::simplify_path("/home/".to_string()));
    println!("{}", Solution::simplify_path("/../".to_string()));
    println!("{}", Solution::simplify_path("/home//foo/".to_string()));
    println!("{}", Solution::simplify_path("/a/./b/../../c/".to_string()));
}
