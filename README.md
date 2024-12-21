# Rust Borrowing Error Example

This repository demonstrates a common error encountered in Rust when working with references: attempting to create both mutable and immutable references to the same variable without adhering to the borrowing rules. The `bug.rs` file contains code that causes a compile-time error due to a borrowing violation, while `bugSolution.rs` shows how to correct this.

## Running the code

1. Clone this repository.
2. Navigate to the repository's directory.
3. Compile and run `bug.rs` (it will result in a compiler error). 
4.  Compile and run `bugSolution.rs` to see the corrected version.

## Understanding the Error

Rust's borrowing rules prevent data races. The code in `bug.rs` attempts to have both a mutable and immutable reference to the same variable simultaneously. This is considered unsafe as concurrent access can lead to unpredictable behavior. The compiler prevents this to maintain data integrity. 