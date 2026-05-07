use std::io; 

fn main(){
    println!("Simple Calculator");
    println!("Available operations: +, -, *, /");
    println!("Enter your expression (e.g., 5 + 4):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input"); 

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3{
        println!("INvalid input. Please follow the format: number operator number");
        return;
    }

    let num1: f64 = match tokens[0].parse(){
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number"); 
            return;
        }
    }; 

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse(){
        Ok(n) => n, 
        Err(_) => {
            println!("Invalid second number.");
            return;
        }
    }; 




}