#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // let _is_true = false; <== start variable name with an understore so rust compiler wont throw errro if its not in use
    let is_true = false;
    let my_grade = 'A';

    println!("Hello World!");
}
