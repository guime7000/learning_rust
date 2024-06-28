use std::io;

fn main() {
    println!("Fibonacci - Displays the Nth term of fibonacci sequence\n");

    loop{

        println!("Enter a positive or null integer value: ");

        let mut nth_term = String::new();
    
        io::stdin()
        .read_line(&mut nth_term)
        .expect("Failed to read line");

        match nth_term.trim().parse::<u64>(){
            Ok(num) => {println!("The {num}th of fibonacci sequence equals : {}", fibonacci(num))},
            Err(_) => println!("Input must be a positive or null integer value !"),
        };

    }
}   

fn fibonacci(term: u64)-> u64{
    let mut second_left_term = 0;
    let mut left_term = 1;
    let mut actual_term = 0;
    let mut counter = 2;
    if term == 0 {
        second_left_term
    } else if term == 1 {
        left_term
    } else{
        while counter <= term {
            actual_term = second_left_term + left_term;
            second_left_term = left_term;
            left_term = actual_term;
            counter +=1;
        }
        actual_term
    }

}
