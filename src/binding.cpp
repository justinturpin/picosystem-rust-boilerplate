#include "picosystem-boilerplate-rs/include/binding.h"

void pen(short int r, short int g, short int b, short int a) {
    picosystem::pen(r, g, b, a);
}

void clear() {
    picosystem::clear();
}

void rect(long int x, long int y, long int w, long int h) {
    picosystem::rect(x, y, w, h);
}

void frect(long int x, long int y, long int w, long int h) {
    picosystem::frect(x, y, w, h);
}

void text(rust::Str message, int32_t x, int32_t y) {
    picosystem::text(std::string(message), x, y);
}

std::unique_ptr<buffer_t> buffer(uint32_t w, uint32_t h, rust::Slice<const uint8_t> data) {
    return std::unique_ptr<buffer_t>(
        picosystem::buffer(w, h, (void*) data.data())
    );
}

void blit(const buffer_t &buffer, int32_t sx, int32_t sy, int32_t sw, int32_t sh, int32_t dx, int32_t dy) {
    // Handle negative x and y values

    if (dx < 0) {
        // We have to increase x and y, and set dx and dy to 0
        sx = -dx;
        dx = 0;
        sw = sw - sx;
    }

    if (dy < 0) {
        sy = -dy;
        dy = 0;
        sh = sh - sy;
    }

    picosystem::blit(
        (buffer_t *)&buffer, sx, sy, sw, sh, dx, dy
    );
}

void set_blend_alpha_enabled(bool enabled) {
    if (enabled) {
        picosystem::blend(picosystem::MASK);
    } else {
        picosystem::blend(picosystem::COPY);
    }
}

bool button(uint32_t b) {
    return picosystem::button(b);
}

bool pressed(uint32_t b) {
    return picosystem::pressed(b);
}

uint32_t time_ms() {
    return picosystem::time();
}

uint32_t time_us() {
    return picosystem::time_us();
}

void wait_vsync() {
    picosystem::_wait_vsync();
}

bool is_flipping() {
    return picosystem::_is_flipping();
}

void flip() {
    picosystem::_flip();
}

void update_gpio() {
    picosystem::_lio = picosystem::_io;
    picosystem::_io = picosystem::_gpio_get();
}
