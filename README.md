# Rust Raw Pointer and Slice Safety Bug

This repository demonstrates a common error in Rust related to using raw pointers and slices. The `bug.rs` file contains code that leads to undefined behavior because it modifies a vector after obtaining its raw pointer. The `bugSolution.rs` file presents a safer approach using safe Rust methods.