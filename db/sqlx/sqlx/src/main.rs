use futures::TryStreamExt;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::types::chrono::NaiveDate;
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

// TODO #[derive(Serialize, Deserialize, Debug, Clone)]
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

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    );

    println!("Init delete database. URL: {}", db_url);
    let s = Command::new("sqlx")
        .arg("database")
        .arg("drop")
        .arg("--database-url")
        .arg(&db_url)
        .arg("-y")
        // The output function will create the final command, which we can use to execute later.
        .output()
        .expect("sqlx command failed to start");
    io::stdout().write_all(&s.stderr).unwrap();

    println!("Init create database. URL: {}", db_url);
    let s = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(&db_url)
        .output()
        .expect("sqlx command failed to start");
    io::stdout().write_all(&s.stderr).unwrap();

    println!("Init create tables. URL: {}", db_url);
    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
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

    println!("Init get all data");
    let db_results = sqlx::query("SELECT * from birthdays")
        .map(|row: PgRow| Birthday {
            id_user: row.get("id_user"),
            date_birthday: row.get("date_birthday"),
        })
        .fetch_all(&db_connection)
        .await
        .unwrap();
    println!("{:?}", db_results);

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
        .await.unwrap();
    println!("{:?}", db_results);

    Ok(())
}
