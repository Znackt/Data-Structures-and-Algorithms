use std::io;

fn main() {
    println!("Enter first number");
    let mut input1 = String::new();
    let _ = io::stdin().read_line(&mut input1);
    let _x: i32 = input1.trim().parse().expect("Please enter a valid integer");

    println!("Enter second number");
    let mut input2 = String::new();
    let _ = io::stdin().read_line(&mut input2);
    let _y: i32 = input2.trim().parse().expect("Please enter a valid integer");

    println!("Enter operator");
    let mut input3 = String::new();
    let _ = io::stdin().read_line(&mut input3);
    let _operator: char = input3.trim().parse().expect("Please enter a valid integer");
    calculator(_x, _y, _operator);
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

//WAP to input week number(1-7) and print day of week name using switch case.
/* fn week_name (x:i32) {
    match x {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thrusday"),
        5 => println!("friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid Input"),
    }
}*/

/* Write a program to input month number and print total number of days in month
using match statements */
/*fn month_days (x:i32) {
    match x {
        1 => println!("January, No of days = 31"),
        2 => println!("Febrary, No of days = 28"),
        3 => println!("March, No of days = 31"),
        4 => println!("April, No of days = 30"),
        5 => println!("May, No of days = 31"),
        6 => println!("June, No of days = 30"),
        7 => println!("July, No of days = 31"),
        8 => println!("August, No of days = 31"),
        9 => println!("September, No of days = 30"),
        10 => println!("October, No of days = 31"),
        11 => println!("November, No of days = 30"),
        12 => println!("December, No of days = 31"),
        _ => println!("Invalid Input , Please enter a number between 1 and 12"),
    }
}*/

/* Write a program to create a calculator that performs basic arithemetic operations
(add, subtract, multiply, divide) using switch case and functions. The calculator
should input two numbers and an operator from user */
fn calculator(x: i32, y: i32, operator: char) {
    match operator {
        '+' => println!("{} + {} = {}", x, y, x + y),
        '-' => println!("{} - {} = {}", x, y, x - y),
        '*' => println!("{} * {} = {}", x, y, x * y),
        '/' => {
            if y != 0 {
                println!("{} / {} = {}", x, y, x / y);
            } else {
                println!("Can't divide by zero");
            }
        }
        '%' => {
            if y != 0 {
                println!("{} % {} = {}", x, y, x % y);
            } else {
                println!("Can't divide by zero");
            }
        }
        _ => println!("Enter valid operator"),
    }
}
