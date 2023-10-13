.PHONY: check
check:
	cargo check

.PHONY: compile
compile:
	cargo build --release --target arm-unknown-linux-gnueabi

.PHONY: deploy
deploy: compile
	cp target/arm-unknown-linux-gnueabi/release/libfrcrs.so javastub/src/main/deploy
	cd javastub; ./gradlew deploy
