#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;

    let int3_u32 = (int_u8 as u32) + (int2_u8 as u32);
}
