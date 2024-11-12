build/all:
	@echo "Building all"
	@cd add && cargo component build
	@cd calc && cargo component build
	@cd app && cargo component build
	@cd sub && \
	go run go.bytecodealliance.org/cmd/wit-bindgen-go generate -o internal/ ../wit/subtractor/world.wit && \
	tinygo build -target=wasip2 -o dist/sub.wasm --wit-package ../wit/subtractor/world.wit --wit-world subtractor main.go

run:
	@wasmtime run wasm/main.wasm

compose:
	@wac plug calc/target/wasm32-wasip1/debug/calc.wasm \
    --plug add/target/wasm32-wasip1/debug/add.wasm \
	--plug sub/dist/sub.wasm \
    -o wasm/composed.wasm
	@wac plug app/target/wasm32-wasip1/debug/app.wasm \
	--plug wasm/composed.wasm \
	-o wasm/main.wasm


exec:
	make build/all && make compose && make run