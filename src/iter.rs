#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut arr_it = [1, 2, 3, 4];

    for val in arr_it.iter() {
        println!("{}", val);
    }

    // arr_it.into_iter()

    let mut iter1 = arr_it.iter();

    println!("1st: {:?}", iter1.next());
}
