#![allow(unused)] // Disable warning

use std::io;
use std::os::windows::thread;
use std::thread::JoinHandle;
// use std::io** => bring in all public items under this library to pecific scope
use rand::Rng; // ext library to generate random numbers
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    pub struct Bank {
        balance: f32,
    };

    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //     the_bank.balance -= amt
    // };

    // let mut bank = Bank { balance: 100.0 };

    // withdraw(&mut bank, 5.00);

    // println!("Balance: {}", bank.balance);

    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.00)
    // }

    // thread::spawn(|| customer(&mut bank)).join().unwrap();

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance: {} Withdraw a smaller amount",
                bank_ref.balance
            )
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdrew {} Current Banlance {}",
                amt, bank_ref.balance
            );
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00)
    };

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}
