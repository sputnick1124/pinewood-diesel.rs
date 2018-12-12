#[derive(Queryable)]
pub struct Participant {
    pub id: i32,
    pub participant_id: String,
    pub car_name: String,
    pub weight: f32,
}
