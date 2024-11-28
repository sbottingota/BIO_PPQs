.PHONY: all clean

# RUSTC=rustc
RUSTC=clippy-driver --rustc
RUSTFLAGS=-C opt-level=3

all: q2a

q2a: q2.rs q2a.rs
	$(RUSTC) $(RUSTFLAGS) q2a.rs

clean:
	rm -rf q2a
