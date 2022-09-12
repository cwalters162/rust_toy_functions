use std::io;

fn main() {

    loop {
        println!("Hello what toy function would you like to run?");
        println!("1. Temperature Converter");
        println!("2. Nth Fibonacci number generator");
        println!("3. The Twelve Days of Christmas");
        println!("0. Quit");


        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 1 {
            temp_converter();
            println!("converter.")
        } else if input == 2 {
            generate_fibonacci();
        } else if input == 3 {
            twelve_days_of_christmas();
        } else if input == 0 {
            break
        } else {
            println!("Please input a valid number.")
        }

    }
}

fn temp_converter() {
    loop {
        println!("What would you like to do?");
        println!("1. Celsius to Fahrenheit.");
        println!("2. Fahrenheit to Celsius.");
        println!("0. Quit.");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            celsius_to_fahrenheit();
        } else if choice == 2 {
            fahrenheit_to_celsius();
        } else if choice == 0 {
            break
        } else {
            println!("Please enter a valid response.")
        }
    }

    fn celsius_to_fahrenheit() {
        loop {
            println!("Please enter the temperature in celsius: ");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");

            let choice: f64 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let value = (choice * (9.0/5.0)) + 32.0;
            println!("{choice}C is {value}F");
            break
        }
    }

    fn fahrenheit_to_celsius() {
        loop {
            println!("Please enter the temperature in fahrenheit: ");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");

            let choice: f64 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let value = (choice - 32.0) * (5.0/9.0);
            println!("{choice}F is {value}C");
            break
        }
    }
}

fn generate_fibonacci() {
    loop {
        println!("Please input the Nth number: ");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Failed to read line");

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = fib(number);
        println!("The {number}th number is {result}");
        break
    }
    fn fib(value: u64) -> u64 {
        if value <= 1 {
            return value
        }
        fib(value - 1) + fib(value - 2)
    }
}

fn twelve_days_of_christmas() {
    let mut lyrics: Vec<&str> = Vec::new();
    for i in 1..12 {
        if i == 1 {
            lyrics.push("A partridge in a pear tree");
            println!("On the first day of christmas my true love gave to me");
        } else if i == 2 {
            lyrics.push("Two turtle doves, and");
            lyrics.rotate_right(1);
            println!("On the second day of christmas my true love gave to me");
        } else if i == 3 {
            lyrics.push("Three french hens");
            lyrics.rotate_right(1);
            println!("On the third day of christmas my true love gave to me");
        } else if i == 4 {
            lyrics.push("Four colly birds");
            lyrics.rotate_right(1);
            println!("On the forth day of christmas my true love gave to me");
        } else if i == 5 {
            lyrics.push("Five gold rings");
            lyrics.rotate_right(1);
            println!("On the fifth day of christmas my true love gave to me");
        } else if i == 6 {
            lyrics.push("Six geese a-laying");
            lyrics.rotate_right(1);
            println!("On the sixth day of christmas my true love gave to me");
        } else if i == 7 {
            lyrics.push("Seven swans a-swimming");
            lyrics.rotate_right(1);
            println!("On the seventh day of christmas my true love gave to me");
        } else if i == 8 {
            lyrics.push("Eight maids a-milking");
            lyrics.rotate_right(1);
            println!("On the eighth day of christmas my true love gave to me");
        } else if i == 9 {
            lyrics.push("Nine ladies dancing");
            lyrics.rotate_right(1);
            println!("On the ninth day of christmas my true love gave to me");
        } else if i == 10 {
            lyrics.push("Ten lords a-leaping");
            lyrics.rotate_right(1);
            println!("On the tenth day of christmas my true love gave to me");
        } else if i == 11 {
            lyrics.push("Eleven pipers piping");
            lyrics.rotate_right(1);
            println!("On the eleventh day of christmas my true love gave to me");
        } else if i == 12 {
            lyrics.push("Twelve drummers drumming");
            lyrics.rotate_right(1);
            println!("On the twelfth day of christmas my true love gave to me");
        }

        for line in &lyrics{
            println!("{line}")
        }
        println!();
    }
}