//use diesel::types::Float8;

#[derive(Queryable)]
pub struct Participant {
    pub id: i32,
    pub name: String,
    pub car_name: Option<String>,
    pub car_weight: f64,
}

use super::schema::participants;

#[derive(Insertable)]
#[table_name="participants"]
pub struct NewParticipant<'a> {
    pub name: &'a str,
    pub car_name: &'a str,
    pub car_weight: f64,
}
