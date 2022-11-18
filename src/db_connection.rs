use postgres::{Client, NoTls};

use chrono::NaiveDate;
use crate::user::{
    UserReply,
};

pub fn establish_postgres_connection() -> Client {
    let database_url = "postgres://aunjum:uxaunjum@localhost/rust-grpc";
    Client::connect(database_url, NoTls).unwrap()
}

// Separate it to Model/ or make another file for this if necessary.
pub fn query_list_of_users(postgres_conn: Client) -> Vec<UserReply> {
    let mut v: Vec<UserReply> = Vec::new();
    // https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.query
    for row in &postgres_conn.query("SELECT * FROM users", &[]).unwrap() {
        let date_of_birth: NaiveDate = row.get(3);
        let user = UserReply {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            date_of_birth: date_of_birth.to_string(),
        };
        v.push(user);
    }
    v
}