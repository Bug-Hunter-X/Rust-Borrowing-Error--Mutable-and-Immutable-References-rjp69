fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    // This is okay because z is immutable. It does not violate the borrowing rules.
    println!("x = {}", z);

    *y = 10; // This is okay because y has mutable access to x
    println!("x = {}", x);

    // The following line is an error because z is an immutable reference to x
    // and the mutable reference y already exists. This violates the borrowing rules.
    // println!("x = {}", z); 
}