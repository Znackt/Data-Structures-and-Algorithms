use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let x: i32 = input.trim().parse().expect("Please enter a valid integer");
    
}

//Check even or Odd
/* fn check_even_odd(x:i32) {
    match x % 2 {
        0 => println!("Even"),
        1 | -1 => println!("Odd"),
        _ => println!("Unexpected value"),
    }
}*/

/*Given marks of student. If the marks are greater than 33 print the result
as pass otherwise fail without using if-else statement */
/* fn pass_fail (x:i32) {
    match x > 33 {
        true => println!("Pass"),
        false => println!("Fail"),
    }
}*/

