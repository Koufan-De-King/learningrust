mod fib;
mod lyrics;
mod temperature;

use std::io;
use lyrics::lyrics;
use fib::fibo;
use temperature::conv;

fn main() {
println!("\nHello user, Welcome!!
How would you like to use my program ?
______________________________________
1)- Compute a fibonacci
2)- Convert a temperature from celsius to fahrenheit
3)- Get the lyrics of the song `Twelve days of christmas`
______________________________________");
    let mut choice = String::new();
    io::stdin()
       .read_line(&mut choice)
       .expect("Failed to read your choice");
    let choice: u32 = choice.trim().parse().expect("Your choice should be 1 or 2\n");
    match choice {
        1 => {  
                println!("Enter a number and I'll give you its fibonacci:  ");
                let mut number = String::new();
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read input");
                let number: u64 = number.trim().parse().expect("Please type a number\n");
                let answer: u64 = fibo(number);
                println!("The fibonacci of {number} is {answer}");
            },
        2 => {
                println!("Enter a temperature in celsius and I'll convert it to fahrenheit");
                let mut temperature = String::new();
                io::stdin()
                    .read_line(&mut temperature)
                    .expect("Failed to read temperature");
                let temperature: f32 = temperature.trim().parse().expect("Please type a tempearature\n");
                let result: f32 = conv(temperature);
                println!("{temperature} °C equals {result} °F");
        },
        3 => {
                println!("\nIt goes: ");
                for i in 1..=12{
                    lyrics(i)
                } 
                println!("The END")
        }
        _ => println!("Invalid choice"),
    }
}
