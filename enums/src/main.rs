// enum IpAddr {
//     V4,
//     V6
// }

// rather than an enum inside a struct, we can put data directly into each enum variant.
// enum IpAddr {
//     V4(String),
//     V6(String)
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }

enum Message {
    Quit,                       // Quit has no data associated with it at all.
    Move { x: i32, y: i32 },    // Move has named fields, like a struct does.
    Write(String),              // Write includes a single String.
    ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values.
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// struct IpAddr {

//     kind: IpAddrKind,
//     address: String
// }

// There is one more similarity between enums and structs:
// just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.
// Here’s a method named call that we could define on our Message enum:

impl Message {
    fn call(&self) {}
}

// Rust doens't have null reference
// so for this, it has Option<T> enum when we want to declare None value.

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    /*
        Rust can infer these types because we’ve specified a value inside the Some variant.
        For absent_number, Rust requires us to annotate the overall Option type: the compiler can’t
        infer the type that the corresponding Some variant will hold by looking only at a None value.
        Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
    */

    let some_number = Some(5);
    let some_char = Some('e');

    // later it will be i32 but now is None.
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // you can't add 'i8 to Option<i8>'
    // let sum = x + y;

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // [ TITLE ] Matching with Option<T>
    // none equals with None and other equals Some(T).

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    // GPT
    /*

    When Not to Use Enums?
        While enums are great, they are not always the right tool. For example:
        Dynamic Variants: If your data requires many possible values (e.g., strings or integers), enums might not be practical.
        Unchanging Types: Use a struct if your data has fixed fields without predefined variants.
    */

    // [ TITLE ] Catch-all Patterns and the _ Placeholder

    let dice_roll = 9;
    // we use other keyword for other posiblities.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),

        // They do same.
        // other => move_player(other),
        _ => (), // () represets to nothing to do.
    }

    // [ TITLE ] Concise Control Flow with if let and let else
    /*
        The if let syntax lets you combine if and let into a less verbose
        way to handle values that match one pattern while ignoring the rest
    */

    // Consider the program in Listing 6-6 that matches on an Option<u8> value in the config_max variable
    // but only wants to execute code if the value is the Some variant.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // Instead, we could write this in a shorter way using if let.
    // The following code behaves the same as the match in Listing 6-6:

    /*  GPT

    if let Some(value) = option {
        println!("The value is {}", value);
    } else {
        println!("No value found.");
    }

    In this case:
    If 'option' is 'Some(value)', the code inside the block runs, and value is available for use.
    If 'option' is 'None', the else block executes.

    */
    let config_max = Some(3u8); // 3u8 -> 3 as u8
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    /*
        If the variable 'coin' contains 'Some(value)', the if let block will run, and value will be accessible inside the block.
        If 'coin' contains 'None', the else block will execute.
    */
    let coin = Some("Quarter");
    if let Some(value) = coin {
        println!("The coin is a {}", value);
    } else {
        println!("No coin found.");
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

// [ TITLE ] Matches Are Exhaustive

// There’s one other aspect of match we need to discuss:
// the arms’ patterns must cover all possibilities.
// Consider this version of our plus_one function, which has a bug and won’t compile:
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }

    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     // it use ; at the end to don't continue and exit the function.
    //     return None;
    // };

    // if state.existed_in(1900) {
    //     Some(format!("{state:?} is pretty old, for America!"))
    // } else {
    //     Some(format!("{state:?} is relatively new."))
    // }

    /*
        To make this common pattern nicer to express,
        Rust has let-else. The let-else syntax takes a pattern on the left side and an expression on the right,
        very similar to if let, but it does not have an if branch, only an else branch.
        If the pattern matches, it will bind the value from the pattern in the outer scope.
        If the pattern does not match, the program will flow into the else arm, which must return from the function.
    */
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(5) => Some(5 + 10),
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}!");
            25
        }
    }
}
