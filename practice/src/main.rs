use std::io;

fn main() {
    loop {
        println!("Choose the tasks that you want to perform.");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fibonacci");
        println!("3. 12 days of Christmas");
        
        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read line");
        let opt: i32 = match opt.trim().parse() {
            Ok(o) => o,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match opt {
            1 => {
                println!("Enter Temperature in Celsius :");

                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read line!");
                let x: f32 = x.trim().parse().expect("Input not a Float");
                let res = cel_2_far(x);

                println!("Temperature in Fahrenheit : {}", res);
                break;
            }
            2 => {
                println!("Enter the number to get the Fibonacci! ");

                let mut n = String::new();
                io::stdin().read_line(&mut n).expect("Failed to read line");
                let n: i32 = n.trim().parse().expect("Input not an Integer");
                if n < 0 {
                    panic!("Invalid number for Fibonacci!");
                }
                println!("Fibonacci of {} is {}", n, fibonacci(n));
                break;
            }
            3 => {
                println!("Presenting 12 days of Christmas song!!!");
                lyrics();
                break;
            }
            _ => {
                println!("Invalid option selected.");
                break; // Exit the loop if an invalid option is selected
            }
        }
    }
}

fn cel_2_far(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn lyrics() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];
    for i in 0..days.len() {
        println!("On the {} day of Christmas,", days[i]);
        println!("my true love gave to me");

        for j in (0..=i).rev() {
            println!("{}", gifts[j]);
        }
        println!(); //empty lines between verses
    }
}
