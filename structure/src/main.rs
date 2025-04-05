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
// implement debug mode to print struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// [ TITLE ] implements function for Rectangle struct
/*
    To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle.
    Everything within this impl block will be associated with the Rectangle type.
    Then we move the area function within the impl curly brackets and change the first (and in this case, only)
    parameter to be self in the signature and everywhere within the body.
*/
impl Rectangle {
    // we use &self instead of rectangle: &Rectangle. The &self is actually short for self: &Self.
    // Within an impl block, the type Self is an alias for the type that the impl block is for.
    // &self is shorthand of 'self: &Self'
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn _test(self: &Self) -> () {
        println!("This is a test text");
    }

    fn change_width(&mut self, width: u32) -> () {
        self.width = width;
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width) && (self.height > other.height)
    }
}

// [ TITLE ] Multiple impl Blocks
// Each struct is allowed to have multiple impl blocks
// There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.
impl Rectangle {
    // it's constructor and will return type which impl defined for
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Mohammad"),
        email: String::from("izanloomoha@gmail.com"),
        sign_in_count: 0,
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
    let _user3 = User {
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
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // [ TITLE ] Unit-Like Structs Without Any Fields
    let _subject = AlwaysEqual;

    // ---------------------------------------------------------------------------------------
    // [ TITLE ] An Example Program Using Structs

    let width1 = 30;
    let height1 = 50;

    println!(
        "With variable\n\tThe area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // refactor our project with tuples
    let rect1 = (30, 50);
    println!(
        "With Tuples\n\tThe area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // refactor with struct
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    // we passed rect struct as reference to be valid after print.
    // if I passed directly, its ownership will moved
    // then rect3 wont be valid after print.
    // reference can not move ownership.
    println!(
        "With struct\n\tThe area of the rectangle is {} square pixels.",
        area3(&rect3)
    );

    // [ TITLE ] Print struct
    // first we add #[derive(Debug)] before our stuct definition
    // and use below syntax for printing.
    println!("{rect3:?}");
    println!("{rect3:#?}"); // it more prettier than previous.

    /*
        Another way to print out a value using the Debug format is to use the dbg! macro,
        which takes ownership of an expression (as opposed to println!,
        which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code
        along with the resultant value of that expression, and returns ownership of the value.

        Note: Calling the dbg! macro prints to the standard error console stream (stderr),
        as opposed to println!, which prints to the standard output console stream (stdout).
    */
    dbg!(&rect3);

    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect4);

    // [ TITLE ] Defining Methods
    let mut rect5 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("Area of rect 5 is {}", rect5.area());

    rect5.change_width(10);

    println!("Area of rect 5 is {}", rect5.area());

    /*
        we can use a field within a method of the same name for any purpose.
        In main, when we follow rect1.width with parentheses, Rust knows we mean the method width.
        When we don’t use parentheses, Rust knows we mean the field width.
    */
    if rect5.width() {
        println!("The rectangle has a nonzero width; it is {}", rect5.width);
    }

    // [ TITLE ] Methods with More Parameters
    let rect6 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect7 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect8 = Rectangle {
        width: 60,
        height: 45,
    };

    // we can passed other rectangle reference to struct function to do something.
    println!("Can rect1 hold rect2? {}", rect6.can_hold(&rect7));
    println!("Can rect1 hold rect3? {}", rect6.can_hold(&rect8));

    // [ TITLE ] Associated Functions
    // All functions defined within an impl block are called associated functions
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    // it uses :: namespace for declaring.
    let sqr = Rectangle::square(20);
    println!("{:?}", sqr);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimestions: (u32, u32)) -> u32 {
    dimestions.0 * dimestions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn _build_user(email: String, username: String) -> User {
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
