
use super::schema::rustaceans;


#[derive(serde::Serialize,serde::Deserialize,Queryable)]
pub struct Rustaceans{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}


#[derive(serde::Deserialize,Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustaceans{
    pub name:String, 
    pub email: String,
}