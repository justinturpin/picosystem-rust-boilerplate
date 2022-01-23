cmake_minimum_required(VERSION 3.12)

link_directories(${CMAKE_CURRENT_LIST_DIR}/target/thumbv6m-none-eabi/release)

# Pull in PICO SDK (must be before project)
include(../pico-sdk/pico_sdk_init.cmake)

project(my_project C CXX ASM)

pico_sdk_init()

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# Find the PicoSystem SDK
find_package(PICOSYSTEM REQUIRED)

# Set up your project and sources
add_executable(
  my_project
  main.cpp
)

# Pull in pico libraries that we need
target_link_libraries(my_project picosystem picosystem_boilerplate_rs)

# Enable link time optimizations (LTO)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)

add_link_options(-fuse-ld=lld)

# create map/bin/hex file etc.
pico_add_extra_outputs(my_project)

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/my_project.uf2 DESTINATION .)

pixel_double(my_project)
no_spritesheet(my_project)
no_main(my_project)
