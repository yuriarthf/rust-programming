use std::io::{stdin, stdout, Write};

fn main() {
    print!("Please input the first operand: ");
    let operand1 = request_operand();

    print!("Please input the operation to be performed: ");
    let operation = request_operation();

    print!("Please input the second operand: ");
    let operand2 = request_operand();

    let op = match operation.as_str() {
        "add" => Operation::Add(operand1, operand2),
        "subtract" => Operation::Subtract(operand1, operand2),
        "multiply" => Operation::Multiply(operand1, operand2),
        "divide" => Operation::Divide(operand1, operand2),
        _ => todo!()
    };

    println!("Result: {}", op.calculate());
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn request_operand() -> f64 {
    let mut input = String::new();
    loop {
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        match input.trim_end().parse::<f64>() {
            Ok(num) => {
                return num;
            }
            Err(_) => print!("Invalid input, please choose a number"),
        }
    }
}

fn request_operation() -> String {
    let mut input: String = String::new();
    loop {
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let norm_input = input.trim_end().to_lowercase();
        if let "add" | "subtract" | "multiply" | "divide" = norm_input.as_str() {
            return norm_input;
        } else {
            print!("Invalid operation, please choose between 'add', 'subtract', 'multiply' or 'divide': ");
        }
    }
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Self::Add(a, b) => a + b,
            Self::Subtract(a, b) => a - b,
            Self::Multiply(a, b) => a * b,
            Self::Divide(a, b) => a / b,
        }
    }
}
