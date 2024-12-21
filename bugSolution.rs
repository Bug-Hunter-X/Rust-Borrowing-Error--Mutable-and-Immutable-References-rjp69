fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x using the mutable reference
        println!("x = {}", x); // Print the modified value of x
    } // y goes out of scope here

    let z = &x; // z is an immutable reference to x
    println!("x = {}", z); // Print the value of x after y has gone out of scope
} 