all: build create_build_directory move_release

create_build_directory:
	mkdir -p build

move_release:
	cp ./build/release/rpn ./build/rpn

move_debug:
	cp ./build/debug/rpn ./build/rpn

debug: debug_build create_build_directory move_debug run

build:
	cargo build --release --target-dir build/

run:
	./build/rpn

debug_build:
	cargo build --target-dir build/
