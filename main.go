package main

/*
#cgo LDFLAGS: -L build -ldata
#include "data.h"
*/
import "C"
import (
	"fmt"
)

//export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
//export LD_LIBRARY_PATH=build:$LD_LIBRARY_PATH

func main() {
	// Create a Data instance
	dataInstance := C.createData(5544)

	// Assuming you have a way to access MyData's value,
	// you would need to expose a function to do that in C++.

	// 调用
	C.doSomething(dataInstance)

	// Clean up
	C.destroyData(dataInstance)


	fmt.Println("Data instance created and destroyed successfully.")
}
