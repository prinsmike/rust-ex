// Shadowing enables us to rebind a name to a value of a different type.
//
// It is possible to change the mutability of a binding.
//
// Shadowing a name does not alter or destroy the value it was bound to,
// and the value will continue to exist until it goes out of scope,
// even if it is no longer accessible by any means.

fn main() {
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8"
        let x = 12;
        println!("{}", x); // Prints "12"
    }
    println!("{}", x); // Prints "8"
    let x = 42;
    println!("{}", x); // Prints "42"

    let mut x: i32 = 1;
    x = 7;
    let x = x;  // x is now immutable and is bound to 7.

    let y = 4;
    let y = "I can also be bound to text!"; // y is now of a different type.
}
