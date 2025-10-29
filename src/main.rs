use std::{io, u8};
fn main() {
    // Counvert between Celsius and Fahrenheit
    loop {
        println!("Please input a temperature: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        println!("Is this in Celsius or Fahrenheit?\n");
        println!("1. Celsius");
        println!("2. Fahrenheit");
        print!(">>> ");


        let choice: u8 = loop {
            let mut choice = String::new();
            if io::stdin().read_line(&mut choice).is_err() {
                println!("Failed to read input!");
                continue;
            }
            match choice.trim().parse::<u8>() {
                Ok(n) if n == 1 || n == 2 => break n,
                _ => {
                    println!("Please input 1 or 2.");
                    continue;
                }
            }
        };
        println!("{choice}");

        println!("The chosen scale is {}", if choice == 1 {"Celsius"} else {"Fahrenheit"});
        

        let temp: f32 = input.trim()
            .parse()
            .expect("Not a float!");

        println!("Initial temperature is {temp}°{}", if choice == 1 {"C"} else {"F"});

        let converted: f32;
        if choice == 1 {
            converted = (temp * 9.0/5.0) + 32.0;
        } else {
            converted = (temp - 32.0) * 5.0/9.0;
        }

        println!("Converted temperature is {converted}°{}", if choice == 1 {"F"} else {"C"});
    }
    

}
