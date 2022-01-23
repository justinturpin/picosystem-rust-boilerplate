# Picosystem Rust Boilerplate

I got a Pimoroni Picosystem for Christmas and, finding the Python bindings too
slow for my needs, wanted to see if I could write games in Rust. Here's how I
did it!

## Core Concept

To make use of the official Picosystem SDK, the main application is still C++
(in main.cpp), and the Rust side is a static library that gets linked in. The
C++ simply loads and calls it, after setting up the hardware and the heap
allocator (explained later).

## C++ binding with CXX

I use Cxx instead of Bindgen to create the bindings to the Picosystem SDK.
There's more to type, but it gives me a bit more control.

## External Heap Allocator

Because the Rust side is a library, it doesn't know how to allocate memory on
the heap. The C++ runtime already knows how to allocate memory on the heap
(I think it's part of the Pico SDK), so what I did was teach Rust how to use
C++'s malloc and free. I based it off of huntc's implementation here:

https://gist.github.com/huntc/ab9a505683647aac7bccd2df0fc75f9e

## Alignment for Image Buffers

This one was tricky to figure out. When you do a normal include_bytes! to
include an image buffer, the alignment is just to 8 bits. Picosystem expects
the alignment to be to 16 bits (u16), or it will straight up crash. User ExpHP
came up with a way to make an include_bytes aligned on specific alignment here:

https://users.rust-lang.org/t/can-i-conveniently-compile-bytes-into-a-rust-program-with-a-specific-alignment/24049

## Slightly modified Picosystem SDK

By default the Picosystem SDK includes a main function. I didn't want this,
because it meant I'd have to use static globals for everything and make
unnecessary heap allocations. I modified picosystem/libraries/picosystem.cpp to
change `#ifndef MICROPY_BUILD_TYPE` to `#ifndef NO_MAIN`, then changed its
picosystem.cmake file to include:

```
function(no_main NAME)
  target_compile_options(${NAME} PRIVATE -DNO_MAIN)
endfunction()
```

## Building

You'll need PICOSYSTEM_DIR set to the modified Picosystem SDK, and then
PICO_SDK_PATH set to the normal Pico SDK. Currently the CMakeLists.txt file
expects the pico-sdk to be in the parent directory of this project. I'm new
to CMake so I'll try and fix this.

Building the Rust project can be done with `cargo +nightly build --release`.
Then you build the CMake project in a build directoy. The Makefile shows an
example of how everything can be put together.

## TODO:

- build.rs is a mess that will easily break if the Pico SDK's pathing changed.
- sound! I haven't done any sound bindings.
- this project relies on a LOT of private functions of the Picosystem SDK.
