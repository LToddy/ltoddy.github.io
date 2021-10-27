install:
	@-rustup target add wasm32-unknown-unknown
	@-cargo install trunk wasm-bindgen-cli
	@-cargo fetch
#	@-brew install openssl
#	@-export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/opt/openssl/lib/pkgconfig

run:
	@trunk serve

build:
	@trunk build --release

clean:
	@-trunk clean
