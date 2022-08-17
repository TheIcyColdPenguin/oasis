#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub summary: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Note {
    pub id: i32,
    pub created_on: String,
    pub text: String,

    pub person_id: i32,
}
