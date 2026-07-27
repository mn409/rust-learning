// use core::num;
// use std::io::stdin;

// fn show_menu() {
//     println!("\n------------------------------");
//     println!("1. Add numbers");
//     println!("2. Square number");
//     println!("3. Get age");
//     println!("4. Exit");
//     println!("------------------------------");
// }

// fn get_choice() -> i32 {
//     loop {
//         let mut input = String::new();
//         println!("Enter your choice (1-4):");

//         stdin().read_line(&mut input).unwrap();

//         match input.trim().parse::<i32>() {
//             Ok(num) => return num,
//             Err(_) => println!("Invalid option, try again"),
//         }
//     }
// }

// fn get_number(prompt: &str) -> i32 {
//     loop {
//         let mut input = String::new();
//         println!("{}", prompt);

//         stdin().read_line(&mut input).unwrap();

//         match input.trim().parse::<i32>() {
//             Ok(num) => return num,
//             Err(_) => println!("Invalid input, try again"),
//         }
//     }
// }

// // fn add_numbers() {
// //     let a = get_number("Enter first number:");
// //     let b = get_number("Enter second number:");

// //     println!("Result: {}", a + b);
// }

// fn add_numbers(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn square_number(num: i32) {
//     num * num
// }
// // fn square_number() {
// //     let num = get_number("Enter number to square:");
// //     println!("Result: {}", num * num);
// // }

// fn get_age() {
//     loop {
//         let age = get_number("Enter your age (1-120):");

//         if age >= 1 && age <= 120 {
//             println!("Valid age: {}", age);
//             break;
//         } else {
//             println!("Age must be between 1 and 120");
//         }
//     }
// }

// fn pause() {
//     let mut temp = String::new();
//     println!("\nPress Enter to continue...");
//     stdin().read_line(&mut temp).unwrap();
// }

// fn main() {
//     loop {
//         show_menu();

//         let choice = get_choice();

//         match choice {
//             1 => add_numbers(),
//             2 => square_number(),
//             3 => get_age(),
//             4 => {
//                 println!("Exiting...");
//                 break;
//             }
//             _ => println!("Invalid choice"),
//         }

//         pause();
//     }
// }

use std::io::stdin;




fn main() {
    match get_input() {
        UserInput::Number(n) => println!("You entered: {}", n),
        UserInput::Exit => println!("Exiting..."),
    }
}

enum UserInput {
        Number(i32),
        Exit,
    }

fn get_input() -> UserInput {

    loop {
        let mut input = String::new();
        println!("Enter the number: ");

        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit"{
            return UserInput::Exit;
        }

        match input.trim().parse::<i32>(){
            Ok(num) => return UserInput::Number(num),
            Err(_) => {
                println!("Invaliddddddd");
            }
        }
    }
}