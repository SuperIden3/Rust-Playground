TARGET = main
CARGO_DIR = main
BUILD_DIR = $(CARGO_DIR)/target

.PHONY: all
all: build run

.PHONY: build
build:
	cd $(CARGO_DIR) && cargo build --release --verbose
.PHONY: run
run: build
	echo "" && ./$(BUILD_DIR)/release/$(TARGET)
.PHONY: clean
clean:
	cd $(CARGO_DIR) && cargo clean
