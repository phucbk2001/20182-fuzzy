.PHONY: all
all: 
	cargo build

test:
	cargo test

run:
	cargo run

count: 
	fd | grep ".rs" | xargs wc -l
