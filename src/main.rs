use std::io;

fn main() {
    println!("Guess the number... or else!");

    println!("take a swing at it y dontcha");

    //the value on immutable variables wont change
    //storing values with variables
    let mut guess = String::new(); //mutable - value can change

    //recieving user input
    io::stdin()
        .read_line(&mut guess) //& key allows 'argument' to be a reference
        //means that it allows multiple parts of code to access piece of data without storing it in memory over and over
        .expect("failurrreeee"); //handling potential failure

    //printing values
    println!("choice: {guess}");
}