#[derive(Queryable, Deserialize, Serialize)]
pub struct Transaction {
    pub id: u64,
    pub payee_id: u64,
    pub amount: f64,
    pub timestamp: u64,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Payee {
    pub id: u64,
    pub name: String,
}