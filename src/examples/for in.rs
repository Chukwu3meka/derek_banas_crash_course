#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx: usize = 0;

    for val in arr_2.iter() {
        println!("Val: {}", val)
    }

    println!("Hello World!");
}
