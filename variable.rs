fn main()
{

        let x = 5;        

        // Error occurs, because by default variable in rust is immutable for change you have to add mut keyboard when you declare variable
        // x = x + 1;
        // println!("{}", x);

        // ==================================================================================================

        let mut x = 5;       
        x = 6;

        // ==================================================================================================

        // declare constant in rust
        const PI_VALUE: f32 = 3.14;

        // ==================================================================================================
        
        // shadowing variable

        let y: u32 = 10;
        let y: u32 = y + 10;

        {
                let y: u32 = y * 10;
                // println!("y in block is: {200}", y);  
        }

        // println!("The value of x is: {y}");

        let spaces = "      ";
        let spaces = spaces.len();

        // Error occurs
        // let mut spaces = "      ";
        // spaces = spaces.len();
        // println!("{}", spaces);


        // ==================================================================================================
        // if I don't add : u32, i'll get error because rust need more information for variable type
        let guess: u32 = "42".parse().expect("NULL");
        // let guess = "42".parse().expect("NULL"); Error occurs

        // Scalar Types A scalar type represents a single value like integers, floating-point numbers, Booleans, and characters.
        /*
                Length        Signed          Unsigned
                8-bit   |       i8      |	u8
                16-bit	|       i16     |	u16
                32-bit	|       i32     |	u32
                64-bit	|       i64     |	u64
                128-bit	|       i128    |       u128
                arch	|       isize	|       usize
        
        Additionally, the isize and usize types depend on the architecture of the computer your program is running on
        which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
        
        */
        // integer
        let x_pos: u16 = 10;
        let large: i128 = 20;
        let readable_value: i32 = 1_000_000; // output: 1000000


        // float
        // in rust by default float is f64
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32


        // Numeric Operations
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        // remainder
        let remainder = 43 % 5;
        

        // The Boolean Type
        let t = true;
        let f: bool = false; // with explicit type annotation

        // The Character Type
        
        // note that char type wraps with single qoute
        let ch = 'C';
        let z: char = 'Z'; // with explicit type annotation


        // Compound Types
        // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

        // Tuple type
        // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        // We can unpack tuple to several variable
        let (x, y, _) = tup;
        let (x, y, z) = tup;
        // Error occurs
        // let (x, y) = tup;

        // We can access to tuple element with dot(.) tuple.n
        let first_index = tup.0;
        let second_index = tup.1;

        // create empty tuple
        let empty_tuple = ();
        let empty_tuple: () = (); 


        // The Array Type
        // every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

        let a = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
        
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        // init array
        let a = [3; 5];

        // access item value in array
        let first = a[0];
        let second = a[1];

        // ==================================================================================================

        // ==================================================================================================
        // ==================================================================================================
        // ==================================================================================================
        // ==================================================================================================
        // ==================================================================================================
        // ==================================================================================================
}
