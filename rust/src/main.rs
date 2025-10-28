use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Please input the maximum.");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let max: i32 = match input.trim().parse() {
        Ok(max) => max,
        Err(e) => panic!("Invalid input: {e}")
    };
    // let max: i32 = input.trim().parse()?;
    fizzbuzz_upto(max);
    Ok(())
}
fn fizzbuzz_upto(max: i32) {
    let fizz = 3;
    let buzz = 5;
    let mut counter = 1;
    while counter <= max {
        let response = if counter % fizz == 0 && counter % buzz == 0 {
            "FizzBuzz!"
        } else if counter % fizz == 0 {
            "Fizz!"
        } else if counter % buzz == 0 {
            "Buzz!"
        } else {
            &counter.to_string()
        };
        println!("{}", response);
        counter += 1;
    }
}
