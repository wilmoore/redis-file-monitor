# Variables
BINARY_NAME = redis-file-monitor
VERSION = 1.0.0
TARGET_DIR = target
MACOS_TARGET = x86_64-apple-darwin
LINUX_TARGET = x86_64-unknown-linux-gnu
RPMBUILD_DIR = ~/rpmbuild
BREW_TAP_DIR = ~/homebrew-tap
SPEC_FILE = $(RPMBUILD_DIR)/SPECS/$(BINARY_NAME).spec

# Default target
all: build-macos test-macos build-linux rpm brew

# Build for macOS
build-macos:
	cargo build --release --target $(MACOS_TARGET)

# Test on macOS
test-macos: build-macos
	./$(TARGET_DIR)/$(MACOS_TARGET)/release/$(BINARY_NAME)

# Build for Linux (CentOS target)
build-linux:
	rustup target add $(LINUX_TARGET)
	cargo build --release --target $(LINUX_TARGET)

# Create RPM directories
setup-rpm:
	mkdir -p $(RPMBUILD_DIR)/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Package binary into an RPM
rpm: build-linux setup-rpm
	tar -czvf $(RPMBUILD_DIR)/SOURCES/$(BINARY_NAME)-$(VERSION).tar.gz -C $(TARGET_DIR)/$(LINUX_TARGET)/release $(BINARY_NAME)
	cp redis-file-monitor.spec $(SPEC_FILE)
	rpmbuild -bb $(SPEC_FILE)

# Create a Homebrew formula
brew: build-macos
	mkdir -p $(BREW_TAP_DIR)
	echo 'Creating Homebrew Formula...'

# Clean up build artifacts
clean:
	cargo clean
	rm -rf $(RPMBUILD_DIR) $(BREW_TAP_DIR)

