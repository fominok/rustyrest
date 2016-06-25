
pub fn add(conn: &::postgres::Connection, name: &str, phone: &str) -> ::postgres::Result<u64> {
    return conn.execute("insert into person (name,phone) values ($1, $2)", &[&name, &phone])
}

pub fn del(conn: &::postgres::Connection, ids: &[i32]) -> ::postgres::Result<u64> {
    let stmt = conn.prepare("delete from person where id=$1").unwrap();
    for i in ids {
        try!(stmt.execute(&[i]));
    }
    Ok(0)
}

pub fn show_all<'a>(conn: &'a::postgres::Connection) -> &'a ::postgres::Result<::postgres::rows::Rows>{
    let stmt = conn.prepare("select id, name, phone from person").unwrap();
    &stmt.query(&[])
}
