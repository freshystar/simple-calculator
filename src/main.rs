use std::io;


fn main() {
 let result: f32;
 println!("Enter the first nuumber:");
 let mut num1 = String::new();

 io::stdin().read_line(&mut num1).expect("Failed to read line");
 let num1: f32 = num1.trim().parse().expect("Please type a number");

 println!("Enter the second number:");
 let mut num2 = String::new();
 io::stdin().read_line(&mut num2).expect("Failed to read line");

 let num2: f32 = num2.trim().parse().expect("Please type a number");

 println!("Enter: 1 for +, 2 for -, 3 for *, 4 for /:");

 let mut sign = String::new();
 io::stdin().read_line(&mut sign).expect("Invalid input");
 let sign: i32 = match sign.trim().parse(){
     Ok(num) => num,
     Err(_) => {
        println!("Invalid input");
        return;
     }
 };

match sign {
    1 => result = num1 + num2,
    2 => result = num1 - num2,
    3 => result = num1 * num2,
    4 => result = num1 /  num2,
    _ => { 
        println!("Invalid selection");
        return;
    }
};

println!("The result is: {}", result);


 }

  


// fn sum(num1: f32, num2: f32) -> f32 {
//  let add: f32 = num1 + num2;
//  return add;
// }

// fn subtraction(num1: f32, num2: f32) -> f32 {
//     let subtract: f32 = num1 - num2;
//     return subtract;
// }

// fn multiplication(num1: f32, num2: f32) -> f32 {
//     let multiply: f32 = num1 * num2;
//     return multiply;
// }

// fn division(num1: f32, num2: f32) -> f32 {
//     let divide: f32 = num1 / num2;
//     return divide;
// }


// fn fun_name() {
//     let  num: f32 = 81.0;
//     println!("The squreroot of  {} is {}", num, square_root(num))
// }

// fn square_root(num: f32) -> f32 {
 //       num.sqrt()
 //}