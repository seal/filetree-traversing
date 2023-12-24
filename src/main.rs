use std::fs;
use std::time::Instant;

const MAX_DEPTH: usize = 5;

#[derive(Debug, Clone)]
struct TreeNode {
    data: String,
    children: Vec<Box<TreeNode>>,
}

impl TreeNode {
    fn new(data: String) -> Self {
        TreeNode {
            data,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(Box::new(child));
    }

    fn build_tree(root_path: &str, depth: usize) -> TreeNode {
        if depth == 0 {
            return TreeNode::new("...".to_string());
        }

        let mut root = TreeNode::new(root_path.to_string());
        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let data = path.to_str().unwrap().to_string();
                    let child = TreeNode::build_tree(&data, depth - 1);
                    root.add_child(child);
                }
            }
        }
        root
    }

    fn depth_first_traversal(&self) {
        self.depth_first_traversal_recursive("");
    }

    fn depth_first_traversal_recursive(&self, prefix: &str) {
        println!("{}{}", prefix, self.data);

        for (i, child) in self.children.iter().enumerate() {
            let is_last = i == self.children.len() - 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "   " } else { "|  " });
            child.depth_first_traversal_recursive(&new_prefix);
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let root = TreeNode::build_tree("/home/will", MAX_DEPTH);

    let elapsed_time = start_time.elapsed();
    println!("Tree built in: {:?}", elapsed_time);
    let start_time = Instant::now();
    root.depth_first_traversal();
    let elapsed_time = start_time.elapsed();
    println!("Traversal completed in: {:?}", elapsed_time);
}

