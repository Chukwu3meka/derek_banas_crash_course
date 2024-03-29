#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::ops::Add;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5. 2+ 4.6 = {}", get_sum_gen(5.2, 4.6));
}
