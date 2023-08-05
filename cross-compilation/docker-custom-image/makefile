ROOT_PATH_NAME=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
APP_BINARY_NAME=print-os
APP_DOCKER_IMAGE_NAME=print-os
APP_FOLDER_NAME=print-os
APP_PATH_NAME=$(ROOT_PATH_NAME)/$(APP_FOLDER_NAME)
DOCKER_SHARED_FOLDER_PATH_NAME=/opt/mount
LOCAL_SHARED_FOLDER_PATH_NAME=/tmp


build-for-debian:
	docker build -t $(APP_DOCKER_IMAGE_NAME) $(ROOT_PATH_NAME)
	# https://stackoverflow.com/questions/25292198/docker-how-can-i-copy-a-file-from-an-image-to-a-host
	docker run --rm -v $(LOCAL_SHARED_FOLDER_PATH_NAME):$(DOCKER_SHARED_FOLDER_PATH_NAME) --entrypoint cp $(APP_DOCKER_IMAGE_NAME) /app/$(APP_BINARY_NAME) $(DOCKER_SHARED_FOLDER_PATH_NAME)

