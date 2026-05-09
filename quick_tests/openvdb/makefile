# Variables
BUILD_DIR = build
TARGET = $(BUILD_DIR)/OpenVDBTest

.PHONY: all shell configure build run clean

# Default target: configure, build, and run
all: build run

# Start the Nix development shell
shell:
	nix develop path:.

# Configure the project using CMake
configure:
	cmake -S . -B $(BUILD_DIR)

# Build the project
# Note: This depends on the configure step
build: configure
	cmake --build $(BUILD_DIR) 

# Run the compiled binary
run:
	./$(TARGET)

# Clean up build artifacts
clean:
	rm -rf $(BUILD_DIR)
