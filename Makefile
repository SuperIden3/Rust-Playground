TARGET = main
SRC = src/main.rs
BUILD_DIR = target

.PHONY: all
all: build

.PHONY: build
build:
	cargo build --release
.PHONY: run
run: build
	./$(BUILD_DIR)/release/$(TARGET)
.PHONY: clean
clean:
	cargo clean
