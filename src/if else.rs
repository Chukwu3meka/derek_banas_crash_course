#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut age = 8;

    age = rand::thread_rng().gen_range(0..100);

    if (age >= 1) && (age <= 18) {
        println!("Important Birthday")
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday")
    } else if age >= 65 {
        println!("Important Birthday")
    } else {
        println!("Not an important Birthday")
    }

    let mut my_age = 47;

    let can_vote = if my_age >= 18 { true } else { false };

    println!("Can Vote: {}", can_vote);
}
