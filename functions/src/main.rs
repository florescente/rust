fn main() {

    // Statements do NOT return a value. Ends with ;
    // Expressions can RETURN a value. The value returned doesn't end with ;

    // let x = (let y = 6);
    // gives an error because 'let' is an statement


    // A scope block is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");

    // A function is an expression
    fn plus_one(x: i32) -> i32 {
        x + 1
        // x + 1;
        // this gives an error because the ';' would turn the function into an statement
    }

    let x = plus_one(5);

    println!("The value of x is {x}");

    // if and else are expressions
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The number is {number}");

    // loops are expressions
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // break; is an statement
            // counter * 2 is the value of the loop expression
        }
    };

    println!("The result of the loop is {result}");

    // Everything inside an curly braces can be an expression
}
