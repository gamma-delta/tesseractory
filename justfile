cargo-build:
	cargo build --manifest-path=rust/Cargo.toml

godot:
	RUST_BACKTRACE=1 godot-engine/bin/godot* ./godot/project.godot
