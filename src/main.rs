use std::io;
use std::process;

fn main() {

    loop {
        
        println!("What conversion would you like to do?");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
    
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    
            if choice == 1 {
                println!("Enter the temperature in Fahrenheit");
        
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).expect("Failed to read line");
                let temp: f64 = match temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                };
        
                let celsius = fahrenheit_to_celsius(temp);
                clearscreen::clear().expect("failed to clear screen");
                println!("{:.2} degrees Fahrenheit is {:.2} degrees Celsius", temp, celsius);
            } else if choice == 2 {
                println!("Enter the temperature in Celsius");
        
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).expect("Failed to read line");
                let temp: f64 = match temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                };
        
                let fahrenheit = celsius_to_fahrenheit(temp);
                clearscreen::clear().expect("failed to clear screen");
                println!("{:.2} degrees Celsius is {:.2} degrees Fahrenheit", temp, fahrenheit);
            } else if choice == 3 {
                clearscreen::clear().expect("failed to clear screen");
                process::exit(0);
            } else {
                println!("Invalid choice");
            }
    }


}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}