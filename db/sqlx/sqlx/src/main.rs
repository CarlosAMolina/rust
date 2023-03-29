use futures::TryStreamExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use tokio; // provides `try_next`

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    surname: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:pw@localhost:5432/contacts")
        .await?;
    // Fetch one.
    let row: (i32, String, String) = sqlx::query_as("SELECT * from contacts.contacts.users")
        .fetch_one(&pool)
        .await?;
    println!("{:?}", row);
    // Fetch all rows.
    let mut rows = sqlx::query("SELECT * from contacts.contacts.users").fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        // map the row into a user-defined domain type
        let id = row.try_get("id")?;
        let name = row.try_get("name")?;
        let surname = row.try_get("surname")?;
        let user = User { id, name, surname };
        println!("{:?}", user);
    }
    Ok(())
}
