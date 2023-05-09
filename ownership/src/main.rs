// Rules of ownership

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// Rules of References

//At any given time, you can have either one mutable reference or any number of immutable references.
//References must always be valid.



fn main() {
    let s = String::From("String");
}
