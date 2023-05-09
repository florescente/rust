fn main() {
    // let x = 5;
    // this gives an error: cannot assign twice to immutable variable `x`
    // because all variables are immutable by default
    
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;

    // let x = "batata";
    // by redeclaring a variable we are shadowing the last value with the new
    // shadowing allow us to change the variable type and continue to be immutable afterwards
    // mutate only allow us to change the value with the same type

    println!("The value of x is {x}");

    // constants are always immutable and cannot use mut
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
