fn main() {

    let s1 = String::from("hello");
    // passed &s1 reference to function. this operation doesn't take its ownership.
    // The name of this operation is passed by reference.
    let len = calculates_length(&s1);

    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    // Because the reference does not own it,
    // the value it points to will not be dropped when the reference stops being used.

    println!("The length of '{s1}' is {len}.");

    // in real life when you borrow something from other, after you're done,
    // you have to give it back.
    // so you can't change or modify it.
    // change(&s1);     // it doesn't work.

    let mut s2 = String::from("hello");
    // &mut allows reference to modify s2 like reading input from io.
    change(&mut s2);

    /*
    [ NOTE ]
    Mutable references have one big restriction: 
        if you have a mutable reference to a value, you can have no other references to that value.
        This code that attempts to create two mutable references to s will fail
    
    */
    let mut s = String::from("hello");

    // we cannot borrow s as mutable more than once at a time.
    // let r1 = &mut s;
    // let r2 = &mut s;
    // it'll give you error.
    // println!("{}, {}", r1, r2);


    // but it will work well. but r1 no valid out of this scope.
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;

    /*
    
    The benefit of having this restriction is that Rust can prevent data races at compile time.
    A data race is similar to a race condition and happens when these three behaviors occur:
        Two or more pointers access the same data at the same time.
        At least one of the pointers is being used to write to the data.
        There’s no mechanism being used to synchronize access to the data.
    
    */

    // We also cannot have a mutable reference while we have an immutable one to the same value.
    /*
        Users of an immutable reference don’t expect the value to suddenly change out from under them! However,
        multiple immutable references are allowed because no one who is just
        reading the data has the ability to affect anyone else’s reading of the data.    
    */

    let mut sname = String::from("hello");

    let r1 = &sname; // no problem
    let r2 = &sname; // no problem
    // let r3 = &mut sname; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);


    // You can use reference mutable after immutable reference no valid.
    /* 
    Note that a reference’s scope
    starts from where it is introduced and continues through the last time that reference is used.
    For instance, this code will compile because the last usage of the immutable references
    is in the println!, before the mutable reference is introduced:
    */

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");


    let reference_to_nothing = dangle();

    // Let’s recap what we’ve discussed about references:
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.

}

// These ampersands represent references, 
// and they allow you to refer to some value without taking ownership of it
// When functions have references as parameters instead of the actual values, 
// we won’t need to return the values in order to give back ownership, because we never had ownership.
fn calculates_length(s: &String) -> usize {
        s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// remove & before return type.
fn dangle() -> String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!