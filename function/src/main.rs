fn main() {
    println!("Hello, world!");

    another_function(6);
    labeled(10,'m');
    let x = return_fn();
    println!("The value of x is {x}");
    let y = plus_one(x);
    println!("The value of x is {y}");
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn labeled(value: i32,unit: char){
    println!("The measurement is {value} {unit}");
}

fn return_fn() -> i32 {
    10
}

fn plus_one(x: i32) -> i32{
    x + 1
}