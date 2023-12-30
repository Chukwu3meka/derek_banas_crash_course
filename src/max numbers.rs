#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    println!("Hello World!");
}
