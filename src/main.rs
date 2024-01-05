#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_1: [i32; 4] = [1, 2, 3, 4];

    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx: usize = 0;

    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }

        if arr_2[loop_idx] == 9 {
            break;
        }

        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    println!("1st: {}", arr_1[0]);
    println!("1st: {}", arr_1[0]);
    println!("1st: {}", arr_1[0]);
}
