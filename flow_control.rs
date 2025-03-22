fn main() i{
    let number = 4;

    if number > 5 {
        println!("your number is greater than 5");
    } else {
        println!("your number is lower than 5");
    }

    /*
        We got error, because rust unlike other language doesn't convert non-boolean variable to boolean.
        if number {
            println!("number was three");
        }
    */
    if number != 0 {
        println!("{number}");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement.
    // type of variable in two side of if have to be same.
    // let assign = if 1 != 1 {1} else {"six"}; Error.
    // wraps the return value with braces, because compile translate it a expression.
    let assign = if 1 != 1 { 1 } else { 2 };
    println!("{assign}");
}
