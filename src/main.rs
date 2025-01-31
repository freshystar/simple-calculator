use std::io;
use factorial::factorial;
use std::fs::OpenOptions;
use std::io::Write;


fn main() {
 let result: f64;
 println!("Which operation do you want to run?...Enter: 1 for +, 2 for -, 3 for *, 4 for /, 5 for %, 6 for ^, 7 for !, 8 for power:");

 let mut sign = String::new();
 io::stdin().read_line(&mut sign).expect("Invalid input");
 let sign: i32 = match sign.trim().parse(){
     Ok(num) => num,
     Err(_) => {
        println!("Invalid input");
        return;
     }
 };
 if sign == 7 {
    println!("Enter your number:");
    let mut num1 = String::new();

 io::stdin().read_line(&mut num1).expect("Failed to read line");
 let num1: f64 = num1.trim().parse().expect("Please type a number");
    match sign {
        7 => result = factorial(num1),
        _ => { 
            println!("Invalid selection");
            return;
        }
    };
 } else {
    println!("Enter the first number:");
 let mut num1 = String::new();

 io::stdin().read_line(&mut num1).expect("Failed to read line");
 let num1: f64 = num1.trim().parse().expect("Please type a number");

 println!("Enter the second number:");
 let mut num2 = String::new();
 io::stdin().read_line(&mut num2).expect("Failed to read line");

 let num2: f64 = num2.trim().parse().expect("Please type a number");

 match sign {
    1 => result = sum(num1, num2),
    2 => result = subtraction(num1, num2),
    3 => result = multiplication(num1, num2),
    4 => result = division(num1, num2),
    5 => result = modulus(num1, num2),
    6 => result = exponential(num1, num2),
    8 => result = power(num1, num2),
    _ => { 
        println!("Invalid selection");
        return;
    }
};

log(num1, sign, num2, result);
}

println!("The result is: {}", result);
//log(num1, f64, &sign, num2, result);

}
  mod factorial;
 
fn sum(num1: f64, num2: f64) -> f64 {
 let add: f64 = num1 + num2;
 return add;
}

fn subtraction(num1: f64, num2: f64) -> f64 {
    let subtract: f64 = num1 - num2;
    return subtract;
}

fn multiplication(num1: f64, num2: f64) -> f64 {
    let multiply: f64 = num1 * num2;
    return multiply;
}

fn division(num1: f64, num2: f64) -> f64 {
    let divide: f64 = num1 / num2;
    return divide;
}

fn power(num1: f64, num2: f64) -> f64 {
    let pow: f64 = num1.powf(num2);
    return pow
}

fn exponential(num1: f64, num2: f64) -> f64{
    let expo: f64 = num1 * 10_f64.powf(num2);
    return expo;
}

fn modulus(num1: f64, num2: f64) -> f64 {
    let modu: f64 = num1 % num2;
    return modu;
}

fn log(num1: f64, sign: i32, num2: f64, result: f64) {
    let log_entry = format!("{} {} {} = {}\n", num1, sign, num2, result);
    let mut hist_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("calculator.log")
        .expect("Cannot open file");
    hist_file.write_all(log_entry.as_bytes()).expect("Cannot write to file");
}
