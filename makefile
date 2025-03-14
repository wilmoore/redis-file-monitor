# Project Variables
BIN_NAME = redis-file-monitor
BUILD_DIR = target
TARGET_DIR = $(BUILD_DIR)/release

# Build a release binary
build:
	cargo build --release

# Strip unneeded symbols to reduce size (Linux/macOS)
strip:
	strip $(TARGET_DIR)/$(BIN_NAME) || echo "Skipping strip (unsupported on this OS)"

# Compress the binary using UPX (optional, requires UPX installed)
compress: build
	upx --best --lzma $(TARGET_DIR)/$(BIN_NAME) || echo "UPX compression failed or not installed"

# Package the binary into a .tar.gz archive (Linux/macOS)
package: build strip
	tar -czvf $(BIN_NAME).tar.gz -C $(TARGET_DIR) $(BIN_NAME)

# Package the binary into a .zip archive (Windows)
package-win: build strip
	zip -j $(BIN_NAME).zip $(TARGET_DIR)/$(BIN_NAME).exe

# Clean up build artifacts
clean:
	cargo clean
	rm -f $(BIN_NAME).tar.gz $(BIN_NAME).zip

# Run the program
run:
	cargo run --release -- --watch-dir . --redis-cli redis-cli

# Test the project
test:
	cargo test