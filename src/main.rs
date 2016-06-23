extern crate postgres;
extern crate ini;

#[macro_use]
extern crate clap;

use ini::Ini;
use postgres::{Connection, ConnectParams, ConnectTarget, SslMode, UserInfo};

use std::str::FromStr;

use clap::App;

const HELP: &'static str = "Usage: phonebook COMMAND [ARG]...
Commands:
    add NAME PHONE - create new record;
    del ID1 ID2... - delete record;
    edit ID        - edit record;
    show           - display all records;
    show STRING    - display records which contain a given substring in the name;
    help           - display this help;";

struct Person {
    id: i32,
    name: String, data: Option<Vec<u8>>
}

fn params<'a>() -> (ConnectParams, SslMode<'a>) {
    let conf = Ini::load_from_file("phonebookrc").unwrap();
    let general = conf.general_section();

    let host = general.get("host").unwrap();
    let port = general.get("port").unwrap();
    let sslmode = general.get("sslmode").unwrap();
    let dbname = general.get("dbname").unwrap();
    let user = general.get("user").unwrap();
    let pass = general.get("pass").unwrap();

    let sslmode_ = match sslmode.as_ref() {
        "disable" => SslMode::None,
        "enable" => unimplemented!(),
        _ => panic!("Wrong sslmode")
    };

    let params = ConnectParams {
        target: ConnectTarget::Tcp(host.clone()),
        port: FromStr::from_str(port).ok(),
        user: Some(UserInfo {
            user: user.clone(),
            password: Some(pass.clone())
        }),
        database: Some(dbname.clone()),
        options: vec![]
    };
    (params, sslmode_)

}

fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();

    if let Some(ref add_m) = m.subcommand_matches("add") {
        println!("Add! {}: {}",
                 add_m.value_of("name").unwrap(), add_m.value_of("phone").unwrap())
    }

    if let Some(ref del_m) = m.subcommand_matches("del") {
        for id in del_m.values_of("ids").unwrap(){
            println!("Del! {}", id)
        }
    }

    if let Some(ref edit_m) = m.subcommand_matches("edit") {
        println!("Edit! {}: {}",
                 edit_m.value_of("id").unwrap(), edit_m.value_of("phone").unwrap())
    }

    if let Some(ref show_m) = m.subcommand_matches("show") {
        if let Some(substring) = show_m.value_of("substring") {
            println!("Show! {}", substring)
        } else {
            println!("show all!")
        }
    }

    //if let Some(mode) = m.is_present("mode") {
    //    match mode {
    //        "fast" => println!("We're really going now!"),
    //        "slow" => println!("Awwww, too slow :("),
    //        _      => unreachable!()
    //    }
    //} else {
    //    println!("--mode <MODE> wasn't used...");
    //}
    //let (params, sslmode) = params();
    //let conn = Connection::connect(params, sslmode).unwrap();
    ////conn.execute("CREATE TABLE person (id SERIAL PRIMARY KEY, name VARCHAR NOT NULL, data BYTEA)",&[]).unwrap();
    //let me = Person {
    //    id: 0,
    //    name: "Jesus".to_string(),
    //    data: None
    //};
    //conn.execute("INSERT INTO person (name,data) VALUES ($1, $2)", &[&me.name, &me.data]).unwrap();
    //let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();

    //for row in &stmt.query(&[]).unwrap() {
    //    let person = Person {
    //        id: row.get(0),
    //        name: row.get(1),
    //        data: row.get(2)
    //    };
    //    println!("Found: {}", person.name);
    //}
}
