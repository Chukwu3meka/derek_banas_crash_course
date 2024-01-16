#![allow(unused)] // Disable warning

use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    enum Day {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursay,
        Friday,
        Saturday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;

    match today {
        Day::Sunday => println!("Everyone hates Monday"),
        Day::Monday => println!("Donut day"),
        Day::Tuesday => println!("Hump day"),
        Day::Wednesday => println!("Pay  day"),
        Day::Thursay => println!("Almost Weekend"),
        Day::Friday => println!("Weekend"),
        Day::Saturday => println!("Weekend  "),
    }

    println!("Is today weekend: {}", today.is_weekend());
}
