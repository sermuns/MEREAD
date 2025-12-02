release:
	RUSTFLAGS="-D warnings" cargo build --release
	cargo release --execute $(git cliff --bumped-version | cut -d'v' -f2)

watch:
	cargo watch -i public -x run

serve:
	~/.cargo/bin/http-server -i public

build-release:
  cargo build --release

benchmark: build-release
    hyperfine \
      --shell=none \
      'target/release/stil'

[parallel]
dev: serve watch
