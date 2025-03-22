fn main() {
    let mut number: i32 = 0;

    loop {
        if number == 5 {
            break;
        }

        println!("Again!\n");
        number += 1;
    }

    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}\n");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}\n");

    let mut i = 0;
    while i < 10 {
        println!("index [{0}]", i + 1);
        i += 1;
    }

    println!("\nFOR LOOP");

    let list = [1, 2, 3, 4, 5];
    
    for element in list {
        println!("{element} ");
    }
 
    println!("\nREVERSE FOR LOOP");

    for index in {1..5}.rev() {
        println!("{index}");
    }

}
