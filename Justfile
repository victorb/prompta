install: test install-linux

install-linux: build-release-linux
  cp target/release/prompta ~/bin/prompta

build: build-linux

build-linux:
  cargo build

build-release-linux:
  cargo build --release
  upx target/release/prompta

build-windows:
  cargo build --target=x86_64-pc-windows-gnu

build-release-windows:
  cargo build --release --target=x86_64-pc-windows-gnu
  upx target/release/prompta

test:
  cargo test --release

coverage:
  cargo tarpaulin --no-default-features -o html

clean:
  cargo clean
