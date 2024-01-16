#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PIE: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");

    age = age + 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);

    println!("Hello World!");
}
