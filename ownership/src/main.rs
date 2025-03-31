use std::io;

fn main() {
    // // use String for user io because for allocation it on runtime based on user input.
    // let mut name = String::new();

    // // but here we use &str, because we know this string doesn't change.
    // let msg = "Enter your name: ";

    // println!("{msg}");

    // io::stdin()
    // .read_line(&mut name)
    // .expect("Failed to read!");

    // println!("{name}");

    // // Diffrence between String and str
    // /*
    // "String" is a dynamic and mutable string type that owns its data,
    // while "str" (String Slice) is an immutable reference to a fixed portion of a string and does not own the data.
    // */
    // // String can be mutable, if we'll need to change its value later.
    // let mut s = String::from("hello");

    // // push is for pushing character
    // s.push('1');
    // // push_str is for pushing string.
    // s.push_str(", World!");

    // println!("{s}");

    // Variables and Data Interacting with Move
    // int
    // let mut x: i32 = 5;
    // let y: i32 = x;
    // println!("x: {x}\ty: {y}"); // x: 5    y: 5
    // x = 10;
    // println!("x: {x}\ty: {y}"); // x: 10   y: 5

    // String & &str
    // we can't assign String to &str variable and vice versa.
    // it data type will be String.
    // when we copy String to another variable without datatype, it will set String datatype.
    // s2 -> Stting.

    /*
        String has three parts that will store in stack:
            1 pointer   : points to memory that holds the contents of the string.
            2 len       : length of string
            3 capacity

    */
    // NOTE: we said when variable goes out of scope the 'drop' function run and free variable.
    // so, s1 and s2 try to free one location that they've pointed.
    // this is a double free. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    // let s1 = String::from("hello");

    // ERROR
    // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // let s2  = s1;    // Ownership of the String is moved to s2

    // let s2 = s1.clone();
    // println!("{s1} world!");

    // let s1 = String::from("Hello");
    // let s2 = s1;

    // println!("{}", s1); // This would cause a compile-time error
    // println!("{}", s2); // This works fine

    // &str can be passed to same data type.
    // s3, s4 -> &str, because s4 is copy of s3.
    // let s3 = "hello";
    // let s4 = s3;

    // This is Copilot response:
    /*
    move in rust
    "move" refers to a concept related to ownership and memory management.
    Rust has a unique ownership system, and when a value is "moved," the ownership of that value is transferred
    from one variable to another. After the move, the original variable is no longer valid and cannot be used,
    preventing issues like double free errors or dangling pointers.

    For example:

    fn main() {
        let s1 = String::from("Hello");
        let s2 = s1; // Ownership of the String is moved to s2

        // println!("{}", s1); // This would cause a compile-time error
        println!("{}", s2); // This works fine
    }

    Here, s1 owns the string initially, but when we assign s1 to s2, the ownership is moved to s2, and s1 becomes invalid.
    This move semantics is a key feature of Rust's design, ensuring memory safety without needing a garbage collector.

    */

    // When we assinged existing variable to another value
    // Rust will call the 'drop' function immediately.
    let mut word = String::from("hello");
    word = String::from("ahoy");

    println!("{word}, world!");

    // -----------------------------------------------------------------------------------
    /*

    1. Stack

    What it is: A region of memory that stores data in a last-in, first-out (LIFO) manner.
    What gets stored:
        Simple, fixed-size data types like integers, booleans, and floats.
        Pointers to data on the heap (like for String or Vec).

    2. Heap

    What it is: A region of memory for dynamic, flexible storage that doesn’t follow the stack's strict LIFO rules.
    What gets stored:
        Complex or variable-size data like String, Vec, and Box.
        Owned data that may outlive the current scope.

    */

    // use clone function to deep copy.
    // when we use clone, it will
    let w1 = String::from("hello");
    let w2 = w1.clone();

    println!("w1 = {w1}, w2 = {w2}");

    // unlike String that is dynamic. int data type can be like above example
    // because int is fixed size and never change. so this isn't freed x and it will be valid.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    /*
        what types can implements copy trait?

            All the integer types, such as u32.
            The Boolean type, bool, with values true and false.
            All the floating-point types, such as f64.
            The character type, char.
            Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

        example :
            let a = 'a';
            let b = a;
            println!("{a} {b}"); // it's ok.

    */

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    /*
    If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error.
    These static checks protect us from mistakes.
    Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.
     */

    // println!("{s}");     s doesn't exist, so this gets you error

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the Copy trait,
                   // x does NOT move into the function,
    println!("{}", x); // so it's okay to use x afterward

    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    // Return Values and Scope
    // Returning values can also transfer ownership.

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); 
    /* 
        s2 is moved into takes_and_gives_back, which also  moves its return value into s3
        Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
        happens. s1 goes out of scope and is dropped.  
    */ 

    // for don't lose ownership of variable we have to use reference.
    
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
