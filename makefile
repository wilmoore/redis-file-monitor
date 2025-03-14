MACOS_TARGET = x86_64-apple-darwin

all: build-macos

build-macos:
	cargo build --release --target $(MACOS_TARGET)

clean:
	cargo clean