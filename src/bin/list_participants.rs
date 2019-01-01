extern crate pinewood_diesel;
extern crate diesel;

use self::pinewood_diesel::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use pinewood_diesel::schema::participants::dsl::*;

    let connection = establish_connection();
    let results = participants
        //.select(name)
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
