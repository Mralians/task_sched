_default:
	just --list

build:
    cargo build --release
clean:
    cargo clean
install: build
    cp target/release/task_sched /usr/local/bin
