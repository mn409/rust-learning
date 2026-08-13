// // use core::num;
// // use std::io::stdin;

// // fn show_menu() {
// //     println!("\n------------------------------");
// //     println!("1. Add numbers");
// //     println!("2. Square number");
// //     println!("3. Get age");
// //     println!("4. Exit");
// //     println!("------------------------------");
// // }

// // fn get_choice() -> i32 {
// //     loop {
// //         let mut input = String::new();
// //         println!("Enter your choice (1-4):");

// //         stdin().read_line(&mut input).unwrap();

// //         match input.trim().parse::<i32>() {
// //             Ok(num) => return num,
// //             Err(_) => println!("Invalid option, try again"),
// //         }
// //     }
// // }

// // fn get_number(prompt: &str) -> i32 {
// //     loop {
// //         let mut input = String::new();
// //         println!("{}", prompt);

// //         stdin().read_line(&mut input).unwrap();

// //         match input.trim().parse::<i32>() {
// //             Ok(num) => return num,
// //             Err(_) => println!("Invalid input, try again"),
// //         }
// //     }
// // }

// // // fn add_numbers() {
// // //     let a = get_number("Enter first number:");
// // //     let b = get_number("Enter second number:");

// // //     println!("Result: {}", a + b);
// // }

// // fn add_numbers(a: i32, b: i32) -> i32 {
// //     a + b
// // }

// // fn square_number(num: i32) {
// //     num * num
// // }
// // // fn square_number() {
// // //     let num = get_number("Enter number to square:");
// // //     println!("Result: {}", num * num);
// // // }

// // fn get_age() {
// //     loop {
// //         let age = get_number("Enter your age (1-120):");

// //         if age >= 1 && age <= 120 {
// //             println!("Valid age: {}", age);
// //             break;
// //         } else {
// //             println!("Age must be between 1 and 120");
// //         }
// //     }
// // }

// // fn pause() {
// //     let mut temp = String::new();
// //     println!("\nPress Enter to continue...");
// //     stdin().read_line(&mut temp).unwrap();
// // }

// // fn main() {
// //     loop {
// //         show_menu();

// //         let choice = get_choice();

// //         match choice {
// //             1 => add_numbers(),
// //             2 => square_number(),
// //             3 => get_age(),
// //             4 => {
// //                 println!("Exiting...");
// //                 break;
// //             }
// //             _ => println!("Invalid choice"),
// //         }

// //         pause();
// //     }
// // }

// // use core::num;
// // use std::io::stdin;

// // use UserInput::Number;


// // enum UserInput {
// //         Number(i32),
// //         Exit,
// //     }

// // fn main() {
// //     let mut sum = 0;
// //     loop {
// //         match get_input() {
// //             UserInput::Number(n) => {
// //                 sum += n;
// //                 println!("added to the club");
// //             }
// //             UserInput::Exit => break,
// //         }
// //     }
// //     println!("sum {}", sum);
// // }

// // fn get_input() -> UserInput {

// //     loop {
// //         let mut input = String::new();
// //         println!("Enter the number: ");

// //         stdin().read_line(&mut input).unwrap();
// //         let input = input.trim();
// //         if input == "exit"{
// //             return UserInput::Exit;
// //         }

// //         match input.parse::<i32>(){
// //             Ok(num) => return UserInput::Number(num),
// //             Err(_) => {
// //                 println!("Invaliddddddd");
// //             }
// //         }
// //     }
// // }

// // use std::io::stdin;

// // fn main() {
// //     let mut account = Bank {
// //         name: "User".to_string(),
// //         account_id: "12345".to_string(),
// //         balance: 500.0,
// //     };

// //     loop {
// //         println!("Enter amount (or type 'exit'):");

// //         let mut input = String::new();
// //         stdin().read_line(&mut input).unwrap();
// //         let input = input.trim();

// //         if input == "exit" {
// //             println!("Exiting...");
// //             break;
// //         }

// //         let amount: f64 = match input.parse() {
// //             Ok(num) => num,
// //             Err(_) => {
// //                 println!("Invalid input. Try again.");
// //                 continue;
// //             }
// //         };

// //        
// //         let result = account.deposit(amount);

// //         match result {
// //             TransactionResult::Success { new_balance, amount } => {
// //                 println!(
// //                     "Transaction successful. Amount: {}, New Balance: {}",
// //                     amount, new_balance
// //                 );
// //             }
// //             TransactionResult::Failure(reason) => match reason {
// //                 FailureReason::InvalidAmount => {
// //                     println!("Invalid amount. Must be > 0");
// //                 }
// //                 FailureReason::InsufficientFunds => {
// //                     println!("Not enough balance.");
// //                 }
// //             },
// //         }
// //     }
// // }

// // struct Bank {
// //     name: String,
// //     account_id: String,
// //     balance: f64,
// // }

// // impl Bank {
// //     fn deposit(&mut self, amount: f64) -> TransactionResult {
// //         if amount <= 0.0 {
// //             return TransactionResult::Failure(FailureReason::InvalidAmount);
// //         }

// //         self.balance += amount;

// //         TransactionResult::Success {
// //             new_balance: self.balance,
// //             amount,
// //         }
// //     }

// //     fn withdraw(&mut self, amount: f64) -> TransactionResult {
// //         if amount <= 0.0 {
// //             return TransactionResult::Failure(FailureReason::InvalidAmount);
// //         }

// //         if self.balance < amount {
// //             return TransactionResult::Failure(FailureReason::InsufficientFunds);
// //         }

// //         self.balance -= amount;

// //         TransactionResult::Success {
// //             new_balance: self.balance,
// //             amount,
// //         }
// //     }
// // }

// // enum TransactionResult {
// //     Success { new_balance: f64, amount: f64 },
// //     Failure(FailureReason),
// // }

// // enum FailureReason {
// //     InvalidAmount,
// //     InsufficientFunds,
// // }

// // use std::io::stdin;

// // fn main() {
// //     let mut account = Bank {
// //         name: "User".to_string(),
// //         account_id: "12345".to_string(),
// //         balance: 500.0,
// //     };

// //     loop {
// //         println!("\nChoose action: deposit / withdraw / balance / exit");

// //         let mut action = String::new();
// //         stdin().read_line(&mut action).unwrap();
// //         let action = action.trim();

// //         match action {
// //             "deposit" => {
// //                 let amount = get_amount();
// //                 let result = account.deposit(amount);
// //                 handle_result(result);
// //             }

// //             "withdraw" => {
// //                 let amount = get_amount();
// //                 let result = account.withdraw(amount);
// //                 handle_result(result);
// //             }

// //             "balance" => {
// //                 println!("Current balance: {}", account.balance);
// //             }

// //             "exit" => {
// //                 println!("Exiting...");
// //                 break;
// //             }

// //             _ => {
// //                 println!("Invalid choice. Try again.");
// //             }
// //         }
// //     }
// // }


// // fn get_amount() -> f64 {
// //     loop {
// //         println!("Enter amount:");

// //         let mut input = String::new();
// //         stdin().read_line(&mut input).unwrap();

// //         match input.trim().parse::<f64>() {
// //             Ok(num) => return num,
// //             Err(_) => println!("Invalid number, try again"),
// //         }
// //     }
// // }

// // fn handle_result(result: TransactionResult) {
// //     match result {
// //         TransactionResult::Success { new_balance, amount } => {
// //             println!("Success! Amount: {}, New Balance: {}", amount, new_balance);
// //         }
// //         TransactionResult::Failure(reason) => match reason {
// //             FailureReason::InvalidAmount => println!("Amount must be > 0"),
// //             FailureReason::InsufficientFunds => println!("Not enough balance"),
// //         },
// //     }
// // }


// // struct Bank {
// //     name: String,
// //     account_id: String,
// //     balance: f64,
// // }

// // impl Bank {
// //     fn deposit(&mut self, amount: f64) -> TransactionResult {
// //         if amount <= 0.0 {
// //             return TransactionResult::Failure(FailureReason::InvalidAmount);
// //         }

// //         self.balance += amount;

// //         TransactionResult::Success {
// //             new_balance: self.balance,
// //             amount,
// //         }
// //     }

// //     fn withdraw(&mut self, amount: f64) -> TransactionResult {
// //         if amount <= 0.0 {
// //             return TransactionResult::Failure(FailureReason::InvalidAmount);
// //         }

// //         if self.balance < amount {
// //             return TransactionResult::Failure(FailureReason::InsufficientFunds);
// //         }

// //         self.balance -= amount;

// //         TransactionResult::Success {
// //             new_balance: self.balance,
// //             amount,
// //         }
// //     }
// // }
// // enum TransactionResult {
// //     Success { new_balance: f64, amount: f64 },
// //     Failure(FailureReason),
// // }

// // enum FailureReason {
// //     InvalidAmount,
// //     InsufficientFunds,
// // }

// // use std::io::stdin;


// // fn main() {
// //     loop {
// //         println!("\nChoose action: deposit");

// //          let mut action = String::new();
// //          stdin().read_line(&mut action).unwrap();
// //          let action = action.trim();

// //          println!("You typed: {}", action);
// //     };
// // }

// // use core::num;
// // use std::io::stdin;


// // fn check_number(num: i32) -> Option<i32> {
// //     if num > 0 {
// //         Some(num)
// //     } else {
// //         None
// //     }
// // }

// // fn main() {

// //     let mut number = String::new();
// //     println!("Enter number");
// //     stdin().read_line(&mut number).unwrap();

// //     if let Ok(num) = number.trim().parse::<i32>() {
// //         println!("Valid: {}", num);
// //     } else {
// //         println!("Invalid");
// //     }

// // }



// // fn main() {
// //     let mut a = String::from("hello ji");

// //     let b1 = &a;
// //     println!("{}", b1);

// //     let b2 = &mut a;
// // }

// use std::io::stdin;

// fn main() {
    
//     loop {
//     let mut input = String::new();
//     println!("Enter a number bro:");

//     stdin().read_line(&mut input).unwrap();

//     let input = input.trim();

//     if input == "exit" {
//         println!("Exiting...");
//         break;
//     }

//     if let Ok(num) = input.parse::<i32>() {
//         if let Some(n) = check_number(num) {
//             println!("Valid {}", n);
//             break;
//         } else {
//             println!("Invalid (<= 0)");
//         }
//     } else {
//         println!("Invalid input");
//     }
// }
// }

// fn check_number(num: i32) -> Option<i32> {

//     if num > 0 {
//         Some(num)
//     } else {
//         None
//     }
// }

// fn main() {
//     let s = String::from("add");

//     let op1 = s;
//     let op2 = op1.clone();

//     println!("{}", op2);
// }

// fn print_op(op: &str) {
//     println!("{}", op);
// }

// fn main() {
//     let op = String::from("add");

//     let r1 = &op;
//     let r2 = &op;

//     print_op(r1);
//     print_op(r2);
// }

// fn main() {
//     let mut s = String::from("hi");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}", r1);
//     println!("{}", r2);

//     let r3 = &mut s;

// }

// struct Book {
//     title: String,
//     page_no: u32,
//     is_read: bool,
// }

// let my_book = Book {
//     title: String::from("The hobbit"),
//     pages: 310,
//     is_read: bool,
// }

// println!("{}", my_book.title);


// struct Student {
//     name: String,
//     age: u32,
//     grade: char,
// }

// impl Student {
//     fn show(&self) {
//         println!("{}, {}, {}", self.name, self.age, self.grade);
//     }
// }

// fn main() {

//     let student = Student {
        
//         name: String::from("mohan"),
//         age: 23,
//         grade: 'B',
//     };  
//     // println!(" {}, {}, {}", student.name, student.age, student.grade);4
//     student.show();
// }


struct Car {
    brand: String,
    speed: u32,
    is_running: bool,
}

impl Car {
    fn show(&self) {
        println!("{}", self.brand);
        println!("{}", self.speed);
        println!("{}", self.is_running);
    }

    fn start_engine(&mut self) {
        self.is_running = true;
    }

    fn new(brand: String, speed: u32) -> Car {
        is_running = false;
    }
}

fn main() {

    let mut car = Car {
        brand: String::from("mitsubisi"),
        speed: 120,
        is_running: false,
    };

    car.show();
    car.start_engine();
    car.show();

    let car2 = Car::new() {
        println!("{}", car2)
    }
}