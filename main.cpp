/**
 * Loads the Rust library containing the game logic and then runs it.
 **/

#include <math.h>
#include "picosystem.hpp"

extern "C" {
    void configure_heap(void* (*alloc)(unsigned int), void (*dealloc)(void*));
    void game_main();
}

int main() {
    picosystem::_init_hardware();
    picosystem::_start_audio();
    picosystem::backlight(75);

    picosystem::_flip();
    // Wait for the DMA transfer to finish
    while (picosystem::_is_flipping());
    // wait for the screen to update
    picosystem::_wait_vsync();
    picosystem::_wait_vsync();

    configure_heap(malloc, free);
    game_main();
}
