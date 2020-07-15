//use std::{thread, time};
use std::io;

fn main() {
    let mut bpm = 0;
    loop {
        println!("Enter bpm: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                bpm = num;
                break;
            },
            Err(err) => println!("{}", err),
        };
    }

    println!("{}", bpm);
    //thread::sleep(time::Duration::from_millis(500));
}
