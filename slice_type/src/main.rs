fn main() {
    
    // Slice type doesn't have ownership
    // it referenced to part of String.

    let mut s = String::from("Hello, World!");
    // let x = first_word(&s);
    // s.clear();

    // print!("{}", x);

    // slice string
    // [start_point..end_point]
    // start included and end excluded
    let s1 = &s[..6].trim();        // hello
    let s2 = &s[0..6].trim();       // hello
    
    let s3 = &s[6..].trim();        // world
    let s4 = &s[6..s.len()].trim(); // world

    let word = first_word(&s);
    
    // s.clear();  it'll give you compiler error, because we use immutable variable
    // after mutable operation
    
    print!("{}", word);


    let my_string = String::from("Hello World!");
    
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]); // [..] means all string.

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    // ===============================================================================

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);


    // array slice

    let a = [1, 2, 3, 4, 5];
    let slice = &a[2..];

    assert_eq!(slice, &a[2..]);
    
}

// here we change &String to &str to passed both String and &str.
// If we have a string slice, we can pass that directly. 
// If we have a String, we can pass a slice of the String or a reference to the String.

// fn first_word(str: &String) -> &str {
fn first_word(str: &str) -> &str {

    // without slice type
    // let bytes = str.as_bytes();

    // for (index, &value) in bytes.iter().enumerate() {

    //         if value == b' ' {
    //             return index;
    //         }
    //     }

    // str.len()

    let bytes = str.as_bytes();

    for (index, &value) in bytes.iter().enumerate() {

            if value == b' ' {
                return &str[..index];
            }
        }

    &str[..]
}

// fn second_word(str: &String) -> (usize, usize) {
    
// }

// fn first_word(str: &String, count: i32) -> usize {

//     let bytes = str.as_bytes();
//     let mut word_counter = 0;

//     for (index, &value) in bytes.iter().enumerate() {

//             if value == b' ' {
//                 word_counter += 1;
//             }

//             if word_counter == count {
//                 return index;
//             }
//         }

//     str.len()
// }