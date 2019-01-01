extern crate diesel;
extern crate pinewood_diesel;

use self::diesel::prelude::*;
use self::models::*;
use self::pinewood_diesel::*;
use std::io::stdin;

fn main() {
    use pinewood_diesel::schema::participants::dsl::*;

    let connection = establish_connection();

    let users = participants
        //.select(name)
        .load::<Participant>(&connection)
        .expect("Error loading participants");

    println!("Please select the participant you would like to remove:");
    for u in users {
        println!("{}) {}", u.id, u.name);
    }

    let mut input_str = String::new();
    stdin().read_line(&mut input_str).unwrap();
    let trimmed = input_str.trim();
    let selection = trimmed.parse::<i32>().unwrap_or(0);

    let participant: Participant = participants
        .find(selection)
        .first(&connection)
        .expect("ID not found");
    println!("Deleting {}) {}", participant.id, participant.name);

    let _deleted = diesel::delete(participants.find(selection))
        .execute(&connection)
        .expect("Unable to delete participant");
}
