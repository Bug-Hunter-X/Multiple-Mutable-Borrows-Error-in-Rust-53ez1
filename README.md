# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust: attempting to create multiple mutable borrows to the same variable.  The code in `bug.rs` attempts this, resulting in a compile-time error. The corrected version is in `bugSolution.rs`.

Rust's ownership and borrowing system is designed to prevent data races and memory unsafety.  Understanding these concepts is crucial for writing correct and efficient Rust code. This example highlights the importance of respecting Rust's borrowing rules.