// Code generated by wit-bindgen-go. DO NOT EDIT.

package sub

// This file contains wasmimport and wasmexport declarations for "component:subtractor".

//go:wasmexport component:subtractor/sub#sub
//export component:subtractor/sub#sub
func wasmexport_Sub(a0 uint32, b0 uint32) (result0 uint32) {
	a := (uint32)((uint32)(a0))
	b := (uint32)((uint32)(b0))
	result := Exports.Sub(a, b)
	result0 = (uint32)(result)
	return
}
