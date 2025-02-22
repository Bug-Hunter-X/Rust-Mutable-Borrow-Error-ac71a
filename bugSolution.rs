fn main() {
    let mut x = 5;
    { // create a new scope
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
        println!("x = {}", x); // Output: x = 6
    } // the mutable reference is dropped

    let z = &x; // Now it's safe to create an immutable reference
    println!("x = {}", *z); // Output: x = 6
} 