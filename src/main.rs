use std::io;

fn main() {
    // Ask user what temperature would he like to convert
    println!("Please specify if you would like to convert F or C: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim().to_uppercase();

    if choice == "F" {
        println!("Please input a temperature in Fahrenheit:");

        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: i32 = fahrenheit
            .trim()
            .parse()
            .expect("please give me correct string number!");

        let celsius = (fahrenheit - 32) * 5 / 9;

        println!("The temperature in celsius is: {celsius}")
    } else {
        println!("Please input a temperature in Celsius:");

        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius: i32 = celsius
            .trim()
            .parse()
            .expect("please give me correct string number!");

        let fahrenheit = (celsius * 9 / 5) + 32;
        println!("The temperature in fahrenheit is: {fahrenheit}")
    }
}
