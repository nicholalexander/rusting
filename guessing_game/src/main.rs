extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Welcome.  To.  GUESS THE NUMBER!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}.", secret_number);

  loop {
    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .ok()
      .expect("Failed to read line");

    // convert type

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("{} is not a valid guess!", guess.trim() );
        continue;
      }
    };
      

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less    => println!("Too Small!"),
      Ordering::Greater => println!("Too Big!"),
      Ordering::Equal   => {
        println!("You win!");
        break;
      }
    }  
  }
}
