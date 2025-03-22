use std::io;

fn main() {
    

    let a = [1, 2, 3, 4, 5];
    
    println!("enter a number between 1~5");

    // init a mutable string
    let mut index = String::new();

    // we passed &mut index to read_line function. here we say you can borrow variable as mutable
    // variable and modify it.
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");


    // use shadowing to convert and delete space from input(index variable)
    let index: usize = index
        .trim()
        .parse()
        .expect("failed to convert!");

    let element = a[index - 1];

    println!("[{}]: {}", index, element);



}
