ROOT_PATH_NAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
APP_PATH_NAME=$(ROOT_PATH_NAME)/hello-world
BINARY_NAME=hello-world
DOCKER_SHARED_FOLDER_PATH_NAME=/usr/src/myapp
DOCKER_RUST_IMAGE=rust

build-for-debian:
	docker run --rm -v $(APP_PATH_NAME):$(DOCKER_SHARED_FOLDER_PATH_NAME) -w $(DOCKER_SHARED_FOLDER_PATH_NAME) $(DOCKER_RUST_IMAGE) cargo build --release
	mv $(APP_PATH_NAME)/target/release/$(BINARY_NAME) /tmp/
	rm -rf $(APP_PATH_NAME)/target/

run:
	cd hello-world && cargo run

get_hello:
	curl 127.0.0.1:3030/hello/john

get_error_not_found:
	curl 127.0.0.1:3030/hello

test: get_hello \
	get_error_not_found

