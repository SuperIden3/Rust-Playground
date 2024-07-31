TARGET = main
CARGO_DIR = main
BUILD_DIR = $(CARGO_DIR)/target
EXECUTABLE = $(BUILD_DIR)/release/$(TARGET)
DEBUGDIR = $(BUILD_DIR)/release

.PHONY: all
all: run clean

update:
	cd $(CARGO_DIR) && cargo update
compile:
	cd $(CARGO_DIR) && rustc src/main.rs
build:
	cd $(CARGO_DIR) && cargo build --release --verbose
debug: build
	cd $(DEBUGDIR) && rust-gdb -q -tui -ex "break main" -ex "layout split" -ex "run" ./$(DEBUGGABLE)/$(TARGET)
run: build
	./$(EXECUTABLE)
clean:
	cd $(CARGO_DIR) && cargo clean
.PHONY: update build debug compile run clean all