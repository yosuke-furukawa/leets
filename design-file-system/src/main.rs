use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Default, Clone, Debug)]
struct File {
    path: String,
    value: i32,
    children: Box<HashMap<String, File>>,
}

#[derive(Default, Clone, Debug)]
struct FileSystem {
    roots: Box<HashMap<String, File>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSystem {

    fn new() -> Self {
        Self::default()        
    }
    
    fn create_path(&mut self, path: String, value: i32) -> bool {
        let mut nodes = self.roots.as_mut();
        let paths: Vec<&str> = path.split('/').skip(1).collect();

        for (i, p) in paths.iter().enumerate() {
            let key = p.to_string();
            
            match i {
                _ if i == paths.len() - 1 => {
                    match nodes.entry(key.clone()) {
                        Entry::Vacant(o) => { o.insert(File{ path: key, value, children: Box::new(HashMap::new()) }); }
                        _ => return false
                    }
                }
                _ => {
                    match nodes.get_mut(&key) {
                        Some(node) => {
                            nodes = node.children.as_mut();
                        },
                        _ => return false
                    }
                }
            }
        }        
        true
    }
    
    fn get(&self, path: String) -> i32 {
        let mut nodes = &self.roots;
        let mut value = -1;
        for p in path.split('/').skip(1) {
            if let Some(node) = nodes.get(&p.to_string()) {
                nodes = &node.children;
                value = node.value;
            } else {
                return -1;
            }
        }
        value
    }
}

/**
 * Your FileSystem object will be instantiated and called as such:
 * let obj = FileSystem::new();
 * let ret_1: bool = obj.create_path(path, value);
 * let ret_2: i32 = obj.get(path);
 */

fn main() {
    let mut file_system = FileSystem::new();

    println!("{}", file_system.create_path("/leet".to_string(), 1)); // return true
    println!("{}", file_system.create_path("/leet/code".to_string(), 2)); // return true
    println!("{}", file_system.get("/leet/code".to_string())); // return 2
    println!("{}", file_system.create_path("/c/d".to_string(), 1)); // return false because the parent path "/c" doesn't exist.
    println!("{}", file_system.get("/c".to_string())); 
}
