package main

import "sub/internal/component/subtractor/sub"

func init() {
	sub.Exports.Sub = func(x, y uint32) uint32 {
		return x - y
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
