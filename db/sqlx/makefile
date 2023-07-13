CONTAINER_DATA_PATHNAME=/home/postgres/data
CONTAINER_NAME=postgres-container-sqlx
CONTAINER_VOLUME=db_postgres_sqlx
IMAGE_NAME=postgres
IMAGE_VERSION=15.3
POSTGRES_USER=postgres
POSTGRES_PW=pw

create-and-run-docker-db:
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
