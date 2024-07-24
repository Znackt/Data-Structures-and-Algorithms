use std::io;
fn main() {
    println!("Enter number of times");
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: i32 = input.trim().parse().expect("Please enter a valid integer");
    
}

//Print hello world 'n' times. Take 'n' user input!
/* fn hello_world (x:i32) {
    for _i in 0..x {
        println!("Hello World");
    }
}*/

//Print numbers from 1 to 100 
/*fn one_to_hundred (x:i32) {
    for i in 1..=x {
        println!("{}",i);
    }
}*/

//Print all even numbers from 1 to 100
/*fn even_one_to_hundred (x:i32) {
    for i in 0..=x {
        if i%2==0 {
            println!("{}",i);
        }
    }
}*/

//Print the table of 19 
