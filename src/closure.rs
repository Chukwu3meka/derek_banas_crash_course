#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // let var_name = |poaram| -> return_type {BODY}

    // let can_vote = |age: i32| -> bool { age >= 18 };

    // println!("Can Vote: {}", can_vote(8));

    let mut samp1 = 5;

    let print_var = || println!("samp1 = {}", samp1);

    print_var();

    samp1 = 10;

    let mut change_var = || samp1 += 1;

    change_var();

    println!("samp1 = {}", samp1);

    samp1 = 10;

    println!("samp1 = {}", samp1);
}
