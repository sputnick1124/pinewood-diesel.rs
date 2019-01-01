extern crate pinewood_diesel;
extern crate diesel;

use crate::pinewood_diesel::*;
use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    println!("Enter your name:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    println!("Enter your car's name:");
    let mut car_name = String::new();
    stdin().read_line(&mut car_name).unwrap();
    let car_name = car_name.trim();
    println!("Enter your car's weight:");
    let mut car_weight_input = String::new();
    stdin().read_line(&mut car_weight_input).unwrap();
    let trimmed = car_weight_input.trim();
    let mut car_weight = 0f64;
    if let Ok(w) = trimmed.parse::<f64>() {
        car_weight = w;
    }

    let _participant = create_participant(&connection, name, car_name, car_weight);
}
