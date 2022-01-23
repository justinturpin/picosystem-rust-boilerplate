.PHONY: clean build

clean:
	(cd build && make clean)

build:
	cargo +nightly build --release
	mkdir -p build
	(cd build && cmake -DPICOSYSTEM_DIR=${PICOSYSTEM_DIR} .. && make)
	cp build/my_project.uf2 .

sync: build
	cp my_project.uf2 /media/justin/RPI-RP2/
