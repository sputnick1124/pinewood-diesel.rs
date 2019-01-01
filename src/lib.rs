#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{NewParticipant, Participant};

pub fn create_participant<'a>(
    conn: &PgConnection,
    name: &'a str,
    car_name: &'a str,
    car_weight: f64,
) -> Participant {
    use crate::schema::participants;

    let new_participant = NewParticipant {
        name: name,
        car_name: car_name,
        car_weight: car_weight,
    };

    diesel::insert_into(participants::table)
        .values(&new_participant)
        .get_result(conn)
        .expect("Error saving new participant")
}
