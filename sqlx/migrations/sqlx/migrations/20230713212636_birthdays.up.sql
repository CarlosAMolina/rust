-- Add up migration script here
CREATE TABLE IF NOT EXISTS birthdays (
   id_user INT PRIMARY KEY,
   date_birthday DATE NOT NULL
);

