# RUSTC=rustc
RUSTC=clippy-driver --rustc
RUSTFLAGS=-C opt-level=3

.PHONY: all clear

all: q1a q3a q3b

clean:
	rm -f q1a q3a q3b

q1a: q1a.rs q1.rs
q3a: q3a.rs q3.rs
q3b: q3b.rs q3.rs

q1a q3a q3b:
	$(RUSTC) $(RUSTFLAGS) $@.rs

