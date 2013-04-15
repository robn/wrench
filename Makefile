RUSTC ?= rustc
RUSTFLAGS ?= -L ../rust-sdl -L ../rust-opengles

.PHONY: all
all: wavefront

wavefront: wavefront.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@

.PHONY: clean
clean:
	rm -f wavefront
