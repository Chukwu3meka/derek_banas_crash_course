#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn print_str(x: String) {
    println!("A string {}", x)
}

fn print_return_str(x: String) -> String {
    println!("A strong {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn main() {
    let mut str1 = String::from("World");
    // let str2 = str1 // <= will move value of str1
    let str2 = str1.clone();
    // println!("Hello {}", str1);

    // print_str(str1)
    // let str3 = print_return_str(str1);
    // println!("str3 = {}", str3);

    change_string(&mut str1);
}
