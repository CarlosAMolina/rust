run:
	cd hello-world && cargo run

get_hello:
	curl 127.0.0.1:3030/hello/john

get_error_not_found:
	curl 127.0.0.1:3030/hello

test: get_hello \
	get_error_not_found

