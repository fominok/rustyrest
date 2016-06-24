extern crate postgres;
use postgres::{Connection, Result};

pub fn add(conn: &Connection, name: &str, phone: &str) -> Result<u64> {
    return conn.execute("INSERT INTO person (name,phone) VALUES ($1, $2)", &[&name, &phone])
}
