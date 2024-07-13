TARGET = main
CARGO_DIR = main
BUILD_DIR = $(CARGO_DIR)/target

.PHONY: all
all: run clean

.PHONY: update
update:
	cd $(CARGO_DIR) && cargo update --verbose
.PHONY: build
.PHONY: compile
compile: update
	cd $(CARGO_DIR) && rustc src/main.rs
build: update
	cd $(CARGO_DIR) && cargo build --release --verbose
.PHONY: run
run: build
	echo "" && ./$(BUILD_DIR)/release/$(TARGET) && echo ""
.PHONY: clean
clean:
	cd $(CARGO_DIR) && cargo clean
