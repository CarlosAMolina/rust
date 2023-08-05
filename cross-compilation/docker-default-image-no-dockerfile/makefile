ROOT_PATH_NAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
APP_PATH_NAME=$(ROOT_PATH_NAME)/print-os
BINARY_NAME=print-os
DOCKER_SHARED_FOLDER_PATH_NAME=/usr/src/myapp
DOCKER_RUST_IMAGE=rust


build-for-debian:
	docker run --rm -v $(APP_PATH_NAME):$(DOCKER_SHARED_FOLDER_PATH_NAME) -w $(DOCKER_SHARED_FOLDER_PATH_NAME) $(DOCKER_RUST_IMAGE) cargo build --release
	mv $(APP_PATH_NAME)/target/release/$(BINARY_NAME) /tmp/
	rm -rf $(APP_PATH_NAME)/target/

