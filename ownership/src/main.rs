// Rules of ownership

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// Rules of References

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.



fn main() {
    let s1 = String::from("hello");
    // let s2 = s1;
    // this gives an error because the reference was passed from s1 to s2
    let s2 = s1.clone();

    println!("{}, world {}!", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // this doesn't give an error because the size of intergers is known at compile time
    // so the value is simply copied to the y variable

    // types with copy trait: 

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

}
