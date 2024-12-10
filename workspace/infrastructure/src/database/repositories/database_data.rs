pub struct Transaction<T> {
    pub txn: T,
}

#[derive(Debug)]
pub struct DatabaseConnection<T> {
    pub conn: T,
}