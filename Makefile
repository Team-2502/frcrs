.PHONY: compile
compile:
	cd robotcode cargo build --release --target arm-unknown-linux-gnueabi

.PHONY: deploy
deploy: compile
	cp robotcode/target/arm-unknown-linux-gnueabi/release/librobotcode.so javastub/src/main/deploy
	cd javastub; ./gradlew deploy