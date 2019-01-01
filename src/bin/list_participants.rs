extern crate pinewood_diesel;
extern crate diesel;

use crate::pinewood_diesel::*;
use crate::models::*;
use crate::diesel::prelude::*;

fn main() {
    use pinewood_diesel::schema::participants::dsl::*;

    let connection = establish_connection();
    let results = participants
        .load::<Participant>(&connection)
        .expect("Error loading participants");

    for participant in results {
        println!("-------------------");
        println!("Name: {}", participant.name);
        println!("Car Name: {}", participant.car_name.unwrap_or("No name".into()));
        println!("Car Weight: {} g", participant.car_weight);
        println!("");
    }
}
