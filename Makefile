.PHONY: check
check:
	cd frcrs; cargo check

.PHONY: compile
compile:
	cd frcrs; cargo build --release --target arm-unknown-linux-gnueabi

.PHONY: deploy
deploy: compile
	cp frcrs/target/arm-unknown-linux-gnueabi/release/libfrcrs.so javastub/src/main/deploy
	cd javastub; ./gradlew deploy

	
