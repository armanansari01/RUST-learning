use std::io;

fn main(){
    let mut x = 5;
    println!("The value of x is : {x}" );
    x = 6;
    println!("The value of x is : {x}" );

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("Three hours in seconds : {THREE_HOURS_IN_SECONDS}" );


    let y = 6;
    let y = y+1;
    {
        let y = y*2;
        println!("The value of y in the inner scope is : {y}" );
    }
    println!("The value of y is : {y}" );

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{z}" );
    println!("{c}" );
    println!("{heart_eyed_cat}" );

    //creating tuples
    let tup = (500,6.4,1);
    let (p,q,r) = tup;
    println!("The value of q is : {q}");

    let tup1: (i32,f64,u8) = (480,5.7,1);
    let first = tup1.0;
    let second = tup1.1;
    let third = tup1.2;

    print!("{third}");

    let arr = [1,2,3,4,5];
    let fi = arr[0];
    let si = arr[1];

    println!("first element : {fi}" );
    println!("second element : {si}" );

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is : {element}");

}