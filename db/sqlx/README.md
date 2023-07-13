## Previous steps

Create docker PostgreSQL container and volume:

```bash
make run-docker-db
```

## Migrations

### Create migrations files

Although this project has the migrations files, in this section we will see how we created them.

First, install sqlx-cli:

```bash
cargo install sqlx-cli
```

Create migration file:

```bash
cd sqlx
sqlx migrate add -r birthdays
```

Add in the `*_birthdays_table.up.sql` file the table definition:

```bash
CREATE TABLE IF NOT EXISTS birthdays (
   id_user INT PRIMARY KEY,
   date_birthday DATE NOT NULL
);
```

Add in the *_birthdays_table.down.sql file:

```bash
DROP TABLE IF EXISTS birthdays;
```

### Run migrations

Although this project runs the `*.up.sql` migrations files, in this section we will see how to do it manually.

Connect to the database and run migration:

```bash
make connect-psql-db
# First, delete tables created previously.
drop table birthdays;
# Exit the psql command.
make run-migrations
```

The table `_sqlx_migrations` will be created too to keep track of the already run migrations. This table must be deleted in some cases to avoid errors when recreating the other tables.

To run the `*.down.sql` files:

```bash
make revert-migrations
```

## Resources

<https://github.com/launchbadge/sqlx>
