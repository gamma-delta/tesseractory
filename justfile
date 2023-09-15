build-and-run: cargo-build godot

cargo-build:
	cargo build --manifest-path=rust/Cargo.toml

godot:
	RUST_BACKTRACE=1 godot4 ./godot/project.godot
