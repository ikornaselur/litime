lint:
	@touch src/*.rs
	@cargo clippy

docker_build:
	mkdir -p ./artifacts
	docker build -t litime-builder -f builder/Dockerfile builder

build: docker_build
	docker run -it \
		-v `pwd`:/rust/build/litime \
		-v `pwd`/artifacts:/output \
		-e MINIFY="true" \
		litime-builder
	mv ./artifacts/apple/litime ./artifacts/litime-macos
	mv ./artifacts/linux/litime ./artifacts/litime-linux
	mv ./artifacts/windows/litime.exe ./artifacts/litime-win.exe
	rm -rf ./artifacts/apple ./artifacts/linux ./artifacts/windows
