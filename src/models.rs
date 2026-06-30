#[derive(Debug)]
pub struct ClientClasses{
    pub id: String,
    pub name: String,
    pub min_score: Option<i32>,
    pub max_score: i32,
}