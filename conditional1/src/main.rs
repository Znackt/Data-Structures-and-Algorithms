use std::io;
fn main() {
    println!("Please provide an input! ");

    // Create a mutable String to store the input
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let mut x: i32 = input1.trim().parse().expect("Please enter a valid integer");
    grade2(x);
}

/* fn even_odd (x: i32) {
    if x % 2 == 0 {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }
} */

/* fn absolute_value(mut x: i32){
    if x < 0 {
        x = -x;
        println!("{} is the positive of x",x);
    }
    else {
        print!("{} is already positive",x);
    }
} */

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

/*fn check_three_digit (x: i32) {

    if (x > 100) && (x < 999) {
        println!("{} is a 3-Digit number",x);
    }
    else {
        println!("{} is not a 3-Digit number",x);
    }
}*/

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

/*fn check_alphabet(ascii: char) {
match ascii {
      'A'..='Z' => println!("The character you entered is uppercase alphabet"),
      'a'..='z' => println!("The character you entered is lowercase alphabet"),
      _         => println!("The character you entered is not an alphabet"),
    }
}*/

/*fn divisible (x:i32) {

    let y = ((x % 3 == 0) || (x % 5 == 0));
    let z = (x % 15 == 0);
    if z {
        println!("The number you entered is divisible by 15");
    }
    else if y && !z {
        println!("The number you entered is either divisible by 5 or 3 but not by 15");
    }
    else {
        println!("The number you entered is neither divisible by 5 or 3");
    }
}*/

/*fn greatest (x: i32, y:i32, z:i32) {
    if (x > y) {
        if (z > y) {println!("{} is the greatest",z);}

        else {println!("{} is the greatest",x);}
    }
    else {println!("{} is the greatest",y);}
}*/

/* fn grade(x: i32) {
    if (x >= 91) && (x <= 100) {
        println!("Excellent");
    } else {
        if (x >= 81) && (x <= 90) {
            println!("very good");
        } else {
            if (x >= 71) && (x <= 80) {
                println!("good");
            } else {
                if (x >= 61) && (x <= 70) {
                    println!("Can do better");
                } else {
                    if (x >= 51) && (x <= 60) {
                        println!("Average");
                    } else {
                        if (x >= 40) && (x <= 50) {
                            println!("Below Average");
                        } else {
                            println!("Fail")
                        }
                    }
                }
            }
        }
    }
}*/

fn grade(x: i32) {
    let msg = match x {
        91..=100 => "Excellent",
        81..=90 => "Very Good",
        71..=80 => "Good",
        61..=70 => "Can do better",
        51..=60 => "Average",
        41..=50 => "Below Average",
        _ => "Fail",
    };
    println!("{msg}")
}

fn grade2(x: i32) {
    println!(
        "{}",
        [
            "Fail",
            "Below Average",
            "Average",
            "Can do better",
            "Good",
            "Very Good",
            "Excellent"
        ][(x.saturating_sub(1) as usize / 10).saturating_sub(3)]
    )
}
