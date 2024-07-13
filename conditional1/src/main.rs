use std::io;
fn main() {
    println!("Please provide an input! ");

    // Create a mutable String to store the input
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
        .expect("Failed to read line");
    /*Trim the input and parse it to an integer
    let mut x: i32 = input1.trim().parse().expect("Please enter a valid integer");
    let mut y: i32 = input2.trim().parse().expect("Please enter a valid integer"); */
    /* let Some(ch) = input1.trim().chars().next() else {
            println!("No valid character entered.");
            return;
    };
    check_alphabet(ch); */
}

//WAP TO TAKE POSITIVE INTEGER INPUT AND TELL IF IT'S EVEN OR NOT ?

/* fn even_odd (x: i32) {
    if x % 2 == 0 {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }
} */

//GIVEN AN INT. PRINT THE ABSOLUTE VALUE OF THAT INTEGER?

/* fn absolute_value(mut x: i32){
    if x < 0 {
        x = -x;
        println!("{} is the positive of x",x);
    }
    else {
        print!("{} is already positive",x);
    }
} */

/*IF CP AND SP OF AN ITEM IS INPUT THROUGH THE KEYBOARD. WRITE A PROGRAM IF SELLER HAS 
MADE PROFIT FOR LOSS. ALSO DETERMINE HOW MUCH PROFIT OR LOSS INCURED.*/

/*fn profit_loss(x: i32,y: i32){

    //X is cost price & Y is selling price
    if y > x { //selling price > cost price
        println!("The seller made a profit {}",y-x);
    }
    else if x == y {
        println!("The seller made neither profit or loss");
    }
    else {
        println!("The seller made a loss of {}",x-y);
    }
}*/

//Take positive no. Input and tell if it is 3-digit no. or not!

/*fn check_three_digit (x: i32) {

    if (x > 100) && (x < 999) {
        println!("{} is a 3-Digit number",x);
    }
    else {
        println!("{} is not a 3-Digit number",x);
    }
}*/

//Take positive integer input and tell if it's divisible by 5 and 3

/*fn divisible (x: i32) {
    if (x % 3 == 0) && (x % 5 == 0) {
        println!("{} is divisible by both 5 and 3",x);
    }
    else if (x % 3 == 0) || (x % 5 == 0) {
        println!("{} is divisible by either 5 or 3",x);
    }
    else {
        println!("{} is  not divisible by either 5 or 3",x);
    }
}*/

//To check whether character is an alphabet or not 


/*fn check_alphabet(ascii: char) {
    match ascii {
          'A'..='Z' => println!("The character you entered is uppercase alphabet"),
          'a'..='z' => println!("The character you entered is lowercase alphabet"),
          _         => println!("The character you entered is not an alphabet"),
        }
    }*/
    