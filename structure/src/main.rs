struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// [ TITLE ] Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// [ TITLE ] Unit-Like Structs Without Any Fields
struct AlwaysEqual;

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("Mohammad"),
        email: String::from("izanloomoha@gmail.com"),
        sign_in_count: 0
    };

    // If the instance is mutable, 
    // we can change a value by using the dot notation and assigning into a particular field.
    user1.active = false;
    
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    // As with any expression, we can construct a new instance of the struct as the last expression
    // in the function body to implicitly return that new instance.

    // [ Title ] Creating Instances from Other Instances with Struct Update Syntax
    // without stuct update
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // with stuct update
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // as you can see we used username's user1 so it moved to user3 after that we can't use user1.username
    // because it moved, so user1 won't valid.
    // If we had given user2 new String values for both email and username,
    // we just used 'active' & 'sign_in_count' that's stack-only data that have Copy trait
    // then user1 would still be valid after creating user2.
    // but Strings are stored in heap. when move it will be invalid.

    // print!("{}", user1.username);
    
    // [ TITLE ] Using Tuple Structs Without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // [ TITLE ] Unit-Like Structs Without Any Fields
    let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }
    
    // [ Title ] Using the Field Init Shorthand
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

    /*
        Here, we’re creating a new instance of the User struct, which has a field named email.
        We want to set the email field’s value to the value in the email parameter of the build_user function.
        Because the email field and the email parameter have the same name,
        we only need to write email rather than email: email.
    */

}
