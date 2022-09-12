use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn shadow_test() {
    let x = 5;
    println!("x: {}", x);
    let x = x * 2;
    println!("x: {}", x);
    let x = 17;
    println!("x: {}", x);
}

fn magic_number() {
    println!("Guess the Number!");
    // create number to be guessed
    let secret_number = rand::thread_rng().gen_range(1..101);
    // guess loop
    loop {
        // var to store user's guess
        let mut guess = String::new(); // for some reason it needs to be in the loop
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: u32 = guess.trim().parse().expect("Please type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
        println!("You guessed: {}", guess);
    }

    println!("The Secret Number is: {}", secret_number);
}

fn len_experiment() {
    let spaces = "         ";
    println!("there are {} spaces", spaces.len())
}

fn main() {
    magic_number();
    //shadow_test();
    //len_experiment();
}
