use futures::TryStreamExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::io::{self, Write};
use std::process::Command;
use tokio; // provides `try_next`

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    surname: String,
}

struct Config {
    database_host: String,
    database_name: String,
    database_password: String,
    database_port: u16,
    database_user: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let config = Config {
        database_host: "localhost".to_string(),
        database_name: "test_db".to_string(),
        database_password: "pw".to_string(),
        database_port: 5432,
        database_user: "postgres".to_string(),
    };

    println!("Init create database");
    let s = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(format!(
            "postgres://{}:{}@{}:{}/{}",
            config.database_user,
            config.database_password,
            config.database_host,
            config.database_port,
            config.database_name
        ))
        .output()
        .expect("sqlx command failed to start");
    io::stdout().write_all(&s.stderr).unwrap();

    //let pool = PgPoolOptions::new()
    //    .max_connections(5)
    //    .connect("postgres://postgres:pw@localhost:5432/contacts")
    //    .await?;
    //// Fetch one.
    //let row: (i32, String, String) = sqlx::query_as("SELECT * from contacts.contacts.users")
    //    .fetch_one(&pool)
    //    .await?;
    //println!("{:?}", row);
    //// Fetch all rows.
    //let mut rows = sqlx::query("SELECT * from contacts.contacts.users").fetch(&pool);
    //while let Some(row) = rows.try_next().await? {
    //    // map the row into a user-defined domain type
    //    let id = row.try_get("id")?;
    //    let name = row.try_get("name")?;
    //    let surname = row.try_get("surname")?;
    //    let user = User { id, name, surname };
    //    println!("{:?}", user);
    //}
    Ok(())
}
