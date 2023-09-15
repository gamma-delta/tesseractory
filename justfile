set dotenv-load

cargo-build:
	cargo build --manifest-path=rust/Cargo.toml

godot:
	# RUST_BACKTRACE=1 godot-engine/bin/godot* -v ./godot/project.godot
	RUST_BACKTRACE=1 godot4 -v ./godot/project.godot
