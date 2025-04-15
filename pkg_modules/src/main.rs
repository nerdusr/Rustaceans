/*
The compiler will look for the garden module’s code in these places:
    Inline, within curly brackets that replace the semicolon following mod garden
    In the file src/garden.rs
    In the file src/garden/mod.rs
*/

// if you want to create submodules for garden this rules are true and you have to Observe it.
/*
you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for
the parent module in these places:

    Inline, directly following mod vegetables, within curly brackets instead of the semicolon
    In the file src/garden/vegetables.rs
    In the file src/garden/vegetables/mod.rs
*/

use garden::vegetables::Asparagus;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// to
// use std::io::{self, Write}

// import all public items in collections
use std::collections::*;

/* 
    To show Rust where to find an item in a module tree,
    we use a path in the same way we use a path when navigating a filesystem.
    To call a function, we need to know its path.

    A path can take two forms:
        
        An absolute path is the full path starting from a crate root;
            for code from an external crate, the absolute path begins with the crate name,
            and for code from the current crate, it starts with the literal crate.

        A relative path starts from the current module and uses self, super, or an identifier in the current module.
*/

mod garden;

fn main() {
    let a = Asparagus {};
}
