extern crate pinewood_diesel;
extern crate diesel;

use self::pinewood_diesel::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use pinewood_diesel::schema::participants::dsl::*;

    let connection = establish_connection();
    let results = participants
        .select(participants_id)
        .load::<String>(&connection)
        .expect("Error loading participants");

    for participant in results {
        println!("Name: {}", participant);
    }
}
