fn main() {
    let mut c = 0;

    //basic loop
    let res = loop {
        c += 1;

        if c == 10 {
            break c*2;
        }
    };

    println!("The result is {res}" );    

    let mut count = 0;

    // labeled loops
    'counting_up:loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //while loop
    let a = [10,20,30,40,50];
    let mut index=0;
    while index < 5 {
        println!("The value using while loop is : {}", a[index] );
        index += 1;
    }

    //for loop
    for element in a {
        println!("The value using for loop is : {element}")
    }

    //for loop using range
    for num in (1..10).rev(){
        println!("{num}" );
    }
    println!("LIFTOFF!!!");
}