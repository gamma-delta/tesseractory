set dotenv-load

cargo-build:
	cargo build --manifest-path=rust/Cargo.toml

godot:
	# RUST_BACKTRACE=1 godot-engine/bin/godot* -v ./godot/project.godot
	RUST_BACKTRACE=1 /home/petrak/bin/Godot_v4.2-stable_mono_linux_x86_64/Godot_v4.2-stable_mono_linux.x86_64 -v ./godot/project.godot
