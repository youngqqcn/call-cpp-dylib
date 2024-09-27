.PHONY: build
build: 
	@mkdir -p build && cd build && cmake .. && make

.PHONY: go
go: build
	LD_LIBRARY_PATH=build go run main.go


# 运行 rust调用c++的示例
.PHONY:rust
rust: build
	LD_LIBRARY_PATH=build cargo run


.PHONY: cpp
cpp: build
	LD_LIBRARY_PATH=build ./build/DataExample


.PHONY: python
python: build
	python3 main.py