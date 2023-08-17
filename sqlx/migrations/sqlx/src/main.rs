use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::types::chrono::NaiveDate;
use sqlx::Pool;
use sqlx::{Postgres, Row};
use std::io::{self, Write};
use std::process::Command;
use tokio;

struct Config {
    database_host: String,
    database_name: String,
    database_password: String,
    database_port: u16,
    database_user: String,
}

#[derive(Debug)]
pub struct Birthday {
    pub id_user: i32,
    pub date_birthday: NaiveDate,
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

    let postgres_url = format!(
        "postgres://{}:{}@{}:{}",
        config.database_user, config.database_password, config.database_host, config.database_port,
    );
    println!("Init create postgres connection. URL: {}", postgres_url);
    let postgres_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await?;

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    );

    if exists_database(&config, &postgres_connection).await {
        println!("Init delete database. URL: {}", db_url);
        let command = Command::new("sqlx")
            .arg("database")
            .arg("drop")
            .arg("--database-url")
            .arg(&db_url)
            .arg("-y")
            // The output function will create the final command, which we can use to execute later.
            .output()
            .expect("sqlx command failed to start");
        if command.status.code().unwrap() != 0 {
            panic!("Unsucessful command: {:?}", command);
        }
        io::stdout().write_all(&command.stderr).unwrap();
    }
    println!("Init create database. URL: {}", db_url);
    let command = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(&db_url)
        .output()
        .expect("sqlx command failed to start");
    if command.status.code().unwrap() != 0 {
        panic!("Unsucessful command: {:?}", command);
    }
    io::stdout().write_all(&command.stdout).unwrap();
    if !exists_database(&config, &postgres_connection).await {
        panic!("The database has not been created");
    }

    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("Init create tables. URL: {}", db_url);
    sqlx::migrate!().run(&db_connection).await?;

    println!("Init insert data");
    let birthday = Birthday {
        id_user: 1,
        date_birthday: NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap(),
    };
    sqlx::query(
        "INSERT INTO birthdays (id_user, date_birthday)
        VALUES ($1, $2)",
    )
    .bind(birthday.id_user)
    .bind(birthday.date_birthday)
    .execute(&db_connection)
    .await
    .unwrap();

    get_all_db_data(&db_connection).await;

    println!("Init update data");
    let birthday = Birthday {
        id_user: 1,
        date_birthday: NaiveDate::parse_from_str("2011-01-01", "%Y-%m-%d").unwrap(),
    };
    let db_results = sqlx::query(
        "UPDATE birthdays
        SET date_birthday = $1
        WHERE id_user = $2
        RETURNING id_user, date_birthday",
    )
    .bind(birthday.date_birthday)
    .bind(birthday.id_user)
    .map(|row: PgRow| Birthday {
        id_user: row.get("id_user"),
        date_birthday: row.get("date_birthday"),
    })
    .fetch_one(&db_connection)
    .await
    .unwrap();
    println!("{:?}", db_results);

    println!("Init delete data");
    sqlx::query("DELETE FROM birthdays WHERE id_user = $1")
        .bind(birthday.id_user)
        .execute(&db_connection)
        .await
        .unwrap();

    get_all_db_data(&db_connection).await;
    Ok(())
}

async fn exists_database(config: &Config, connection: &Pool<Postgres>) -> bool {
    println!("Init check database {} exists", config.database_name);
    let database_names: Vec<_> = sqlx::query("SELECT datname FROM pg_database")
        .map(|row: sqlx::postgres::PgRow| row.get::<String, _>("datname").to_string())
        .fetch_all(connection)
        .await
        .unwrap();
    println!("Current databases: {:?}", database_names);
    if database_names.contains(&config.database_name) {
        println!("The database {} exists", config.database_name);
        true
    } else {
        println!("The database {} does not exist", config.database_name);
        false
    }
}

async fn get_all_db_data(db_connection: &Pool<Postgres>) {
    println!("Init get all data");
    let db_results = sqlx::query("SELECT * from birthdays")
        .map(|row: PgRow| Birthday {
            id_user: row.get("id_user"),
            date_birthday: row.get("date_birthday"),
        })
        .fetch_all(db_connection)
        .await
        .unwrap();
    println!("{:?}", db_results);
}
