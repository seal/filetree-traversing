
---

# File System Traversal in Rust

## Overview

This Rust program performs a file system traversal, building a tree-like structure to represent the hierarchy of files and directories on a user's computer. The program uses a recursive approach to explore the entire file system, creating a tree structure with nodes representing directories and files.

## Features

- **Recursive Traversal:** The program recursively traverses the file system starting from the root directory.
- **Tree Structure:** Directories and files are represented as nodes in a tree structure.
- **Depth-First Traversal:** The tree is traversed in a depth-first manner, printing each node's information.
- **Error Handling:** The program handles errors encountered during traversal, printing error messages and continuing the traversal.
- **Elapsed Time Measurement:** The program measures and prints the elapsed time for building the tree and completing the traversal.

## Usage

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/seal/filetree-traversing
   ```

2. **Build and Run the Program:**
   ```bash
   cd filetree-traversing
   cargo run
   ```

   Adjust the program settings or source code as needed.

## Configuration

- **Maximum Depth:**
  You can configure the `MAX_DEPTH` constant in the source code to limit the depth of the recursive traversal. This helps prevent excessive memory usage.

```rust
const MAX_DEPTH: usize = 3; // Set your desired maximum depth
```

## Dependencies

- This program relies on the Rust standard library and does not have external dependencies.


---

