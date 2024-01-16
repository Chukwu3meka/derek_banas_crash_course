#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut st1 = String::new();
    st1.push('A');

    st1.push_str(" word");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from(" q w e r t y y y u u i i i i");

    let mut v1: Vec<char> = st3.chars().collect();

    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char)
    }

    let st4 = "Random String";
    let mut st5 = st4.to_string();
    println!("{}", st5);

    let byte_arr_1 = st5.as_bytes();

    let st6 = &st5[0..6];

    println!("String Length: {}", st6.len());

    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;

    for char in st8.bytes() {
        println!("{}", char);
    }
}
