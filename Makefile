.PHONY: build_all build_injecter build_dll test_injecter test_dll
build_all: build_injecter build_dll
build_injecter:
	cd injecter && cargo build --release

build_dll:
	cd dll && cargo build --release

test_injecter:
	cd injecter && cargo run

test_dll:
	cd dll && cargo run