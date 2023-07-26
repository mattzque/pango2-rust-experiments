export PKG_CONFIG_PATH := /opt/pango2/lib/x86_64-linux-gnu/pkgconfig/:${PKG_CONFIG_PATH}
export LD_LIBRARY_PATH := /opt/pango2/lib/x86_64-linux-gnu/:${LD_LIBRARY_PATH}

build:
	cargo build

run-example:
	cd crates/pango2-alpha-examples/ && cargo run $(filter-out $@,$(MAKECMDGOALS))

