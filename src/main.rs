#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let vec1: Vec<i32> = Vec::new();

    let mut vec2 = vec![1, 2, 3, 4];

    vec2.push(5);
    println!("1st: {}", vec2[0]);

    let second: &i32 = &vec2[1];

    // match vec2.get

    // println!(": {}",);
}
