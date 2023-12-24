use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

const MAX_DEPTH: usize = 5;

#[derive(Debug, Clone)]
struct TreeNode {
    data: PathBuf,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(data: PathBuf) -> Self {
        TreeNode {
            data,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    fn build_tree_parallel(root_path: &Path, depth: usize) -> TreeNode {
        if depth == 0 {
            return TreeNode::new(PathBuf::from("..."));
        }

        let mut root = TreeNode::new(root_path.to_path_buf());
        if let Ok(entries) = fs::read_dir(root_path) {
            let children: Vec<TreeNode> = entries
                .par_bridge()
                .map(|entry| {
                    if let Ok(entry) = entry {
                        TreeNode::build_tree(&entry.path(), depth - 1)
                    } else {
                        TreeNode::new(PathBuf::from("..."))
                    }
                })
                .collect();

            for child in children {
                root.add_child(child);
            }
        }
        root
    }

    fn build_tree(root_path: &Path, depth: usize) -> TreeNode {
        if depth == 0 {
            return TreeNode::new(PathBuf::from("..."));
        }

        let mut root = TreeNode::new(root_path.to_path_buf());
        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let child = TreeNode::build_tree(&path, depth - 1);
                    root.add_child(child);
                }
            }
        }
        root
    }

    fn depth_first_traversal_recursive(&self, prefix: &str) {
        println!("{}{:?}", prefix, self.data.display());

        for (index, child) in self.children.iter().enumerate() {
            let child_prefix = if index == self.children.len() - 1 {
                format!("{}└──", prefix)
            } else {
                format!("{}├──", prefix)
            };

            child.depth_first_traversal_recursive(&format!("{} ", child_prefix));
        }
    }
}

fn main() {
    let start_time2 = Instant::now();
    let root2 = TreeNode::build_tree_parallel(Path::new("/"), MAX_DEPTH);
    let tree_build_parallel = start_time2.elapsed();
    root2.depth_first_traversal_recursive("-");
    println!("Parallel Tree built in: {:?}", tree_build_parallel);
}

