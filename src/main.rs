use std::env;

mod l1;
mod l2;
mod l3;
mod l4;
mod l5;
mod l6;
mod l7;
mod l8;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "lesson1" => l1::lesson1::run(),
            "lesson2" => l2::lesson2::run(),
            "lesson3" => l3::lesson3::run(),
            "lesson4" => l4::lesson4::run(),
            "lesson5" => l5::lesson5::run(),
            "lesson6" => l6::lesson6::run(),
            "lesson7" => l7::lesson7::run(),
            "lesson8" => l8::lesson8::run(),
            // Add more cases as more lessons are added
            _ => println!("Lesson not found"),
        }
    } else {
        println!("Please specify a lesson to run (e.g., 'cargo run lesson1')");
    }
}
