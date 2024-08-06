fn main() {
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4.");
    // } else if number % 3 == 0 {
    //     println!("number is divisiable by 3.");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2.");
    // } else {
    //     println!("number is not divisible by 4, 3 or 2");
    // }

    //_____ Using if in a let statement
    let condition = false;
    let number = if condition { 5 } else { 6 };

    /* The follwoing code case error because  `if` and `else` have incompatible types */
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
