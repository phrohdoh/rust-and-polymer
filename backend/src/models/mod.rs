#[derive(Queryable, Deserialize, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub payee_id: i32,
    pub amount: f32,
    pub timestamp: i32,
    pub memo: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Payee {
    pub id: i32,
    pub name: String,
}