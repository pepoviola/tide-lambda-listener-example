build-GetHello: export CARGO_LAMBDA_FN=get_hello
build-GetHello:
	TARGET_CC=x86_64-linux-musl-gcc  RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/tide-lambda-listener-example $(ARTIFACTS_DIR)/bootstrap

build-PostHello: export CARGO_LAMBDA_FN=post_hello
build-PostHello:
	TARGET_CC=x86_64-linux-musl-gcc  RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/tide-lambda-listener-example $(ARTIFACTS_DIR)/bootstrap
