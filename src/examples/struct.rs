#![allow(unused)] // Disable warning

use std::f32::consts::PI;
use std::io;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // struct Customer {
    //     name: String,
    //     address: String,
    //     balance: f32,
    // };

    // let mut bob = Customer {
    //     name: String::from("Bob Smith"),
    //     address: String::from("555 Main st"),
    //     balance: 234.50,
    // };

    // println!(": {}", bob.address);
    // bob.address = String::from("5050 Main st");

    // println!(": {}", bob.address);
    // struct Rectangle<T, U> {
    //     length: T,
    //     height: U,
    // }

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }

        fn area(&self) -> f32 {
            self.length * self.width
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            (self.length / 2.0).powf(2.0) * PI
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area: {}", rec.area());
    println!("Circ Area: {}", circ.area());
}
