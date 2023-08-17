-- Add up migration script here
CREATE VIEW birthdays_view AS
  SELECT
    birthdays.id_user AS id_user,
    birthdays.date_birthday AS date_birthday,
    123 AS foo
  FROM birthdays
;
