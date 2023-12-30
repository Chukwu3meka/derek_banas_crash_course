#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // Precision
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f32: {}", num_2 + 0.111111111111111);

    // basic  Maths
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;

    println!("5 + 4: {}", num_3 + num_4);
    println!("5 - 4: {}", num_3 - num_4);
    println!("5 * 4: {}", num_3 * num_4);
    println!("5 / 4: {}", num_3 / num_4);
    println!("5 % 4: {}", num_3 % num_4);

    num_3 += 1
}
