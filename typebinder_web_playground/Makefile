all: build_artifacts purge_public create_public copy_artifacts

purge_public:
	rm -rf ./public

create_public:
	mkdir public

build_artifacts:
	cargo make verify
	cargo make build_release

copy_artifacts:
	cp ./index.html ./public/
	cp -r ./assets/ ./public/