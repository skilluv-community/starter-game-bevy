.PHONY: dev run test lint fmt wasm serve-web release clean help

help:
	@echo "Targets: dev / run / test / lint / fmt / wasm / serve-web / release / clean"

dev: run

run:
	cargo run

test:
	cargo test --all-features

lint:
	cargo clippy --all-targets -- -D warnings
	cargo fmt --check

fmt:
	cargo fmt

wasm:
	rustup target add wasm32-unknown-unknown
	cargo install --locked wasm-bindgen-cli
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir web/pkg --target web target/wasm32-unknown-unknown/release/starter-game-bevy.wasm

serve-web: wasm
	@echo "Serving on http://localhost:8080 — Ctrl-C to stop"
	python3 -m http.server 8080 -d web

release:
	cargo build --release

clean:
	cargo clean
	rm -rf web/pkg
