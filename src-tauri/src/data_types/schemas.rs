#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub summary: String,
}
