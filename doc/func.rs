fn main() {
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    // this is a statement because it doesn't return anything
    let grade_a = 1;

    // this is a expresstion because its body return a expression to grade_b
    let grade_b = {
        let temp = 3;
        temp - grade_a
    };

    println!("Grade A: {grade_a}, Grade B: {grade_b}");
    println!("Hello, World");
    another_function(10, 20);
    print_labled_measurement(10, 'K');

    // this is a expresstion
    let y: i32 = {
        let x = 3;
        x + 20
    };
    println!("{y}");

    another_function(five(), six());
}

fn another_function(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn print_labled_measurement(value: i32, unit_char: char) {
    println!("The measurement is: {value}{unit_char} ")
}

// return from function
fn five() -> i32 {
    5
}

fn six() -> i32 {
    // when you use semicolon in the end of return variable, it will change from expression to a statement.
    6;
    5 + 3
}

fn plus_one(x: i32) -> i32 {
    
    /* 
        if put semicolon at the end of expression it will change to statement.

        fn plus_one(x: i32) -> i32 {
         |    --------            ^^^ expected `i32`, found `()`
         |    |
         |    implicitly returns `()` as its body has no tail or `return` expression
         |     x + 1;
         |          - help: remove this semicolon to return this value
    */

    x + 1
}
