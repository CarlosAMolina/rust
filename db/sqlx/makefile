CONTAINER_DATA_PATHNAME=/home/postgres/data
CONTAINER_NAME=postgres-container-sqlx
CONTAINER_VOLUME=db_postgres_sqlx
DB_NAME=test_db
IMAGE_NAME=postgres
IMAGE_VERSION=15.3
MIGRATIONS_RELATIVE_PATHNAME=./sqlx/
POSTGRES_USER=postgres
POSTGRES_PW=pw
RUST_PROJECT_RELATIVE_PATHNAME=./sqlx/
DB_URL=postgresql://$(POSTGRES_USER):$(POSTGRES_PW)@localhost:5432/$(DB_NAME)

run-docker-db:
	# PGDATA: tells PostgreSQL where it should store the database.
	docker pull $(IMAGE_NAME):$(IMAGE_VERSION)
	docker volume create $(CONTAINER_VOLUME)
	docker run \
		--rm \
		-d \
		--name $(CONTAINER_NAME) \
		-p5432:5432 \
		-v $(CONTAINER_VOLUME):$(CONTAINER_DATA_PATHNAME) \
		-e POSTGRES_PASSWORD=$(POSTGRES_PW) \
		-e PGDATA=$(CONTAINER_DATA_PATHNAME) \
		$(IMAGE_NAME)

connect-psql:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER)

connect-psql-db:
	docker exec -it $(CONTAINER_NAME) psql -U $(POSTGRES_USER) $(DB_NAME)

run:
	cd $(RUST_PROJECT_RELATIVE_PATHNAME) && cargo run

run-migrations:
	cd $(MIGRATIONS_RELATIVE_PATHNAME) && sqlx migrate run --database-url $(DB_URL)

revert-migrations:
	# Each revert works on the latests migration.
	# If you have run two `*.up.*` migrations files, you must run this command twice.
	cd $(MIGRATIONS_RELATIVE_PATHNAME) && sqlx migrate revert --database-url $(DB_URL)

