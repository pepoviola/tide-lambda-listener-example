build-GetHello: export CARGO_LAMBDA_FN=get_hello
build-GetHello: export CARGO_LAMBDA_METHOD=get
build-GetHello: export CARGO_LAMBDA_ROUTE=/hello/:name
build-GetHello:
	TARGET_CC=x86_64-linux-musl-gcc  RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/tide-lambda-listener-example $(ARTIFACTS_DIR)/bootstrap

build-PostHello: export CARGO_LAMBDA_FN=post_hello
build-PostHello: export CARGO_LAMBDA_METHOD=post
build-PostHello: export CARGO_LAMBDA_ROUTE=/hello
build-PostHello:
	TARGET_CC=x86_64-linux-musl-gcc  RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/tide-lambda-listener-example $(ARTIFACTS_DIR)/bootstrap
