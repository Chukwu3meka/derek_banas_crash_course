#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::thread;
use std::time::Duration;

fn main() {
    // common problems with parallel programming involve:
    // 1. Thread are accessing data in the wrong order
    // 2. Threads are blocked from executing because of confusion over requirments to ptroceed with execution

    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);

            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1))
    }

    thread1.join().unwrap();
}
