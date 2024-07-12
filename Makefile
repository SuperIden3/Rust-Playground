TARGET = main
CARGO_DIR = main
BUILD_DIR = $(CARGO_DIR)/target

.PHONY: all
all: build run

.PHONY: build
build:
	cd $(CARGO_DIR) && cargo build --release --quiet
.PHONY: run
run: build
	./$(BUILD_DIR)/release/$(TARGET)
.PHONY: clean
clean:
	cd $(CARGO_DIR) && cargo clean
