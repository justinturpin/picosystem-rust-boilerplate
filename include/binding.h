#pragma once

#include "rust/cxx.h"
#include "picosystem.hpp"
#include <memory>

using picosystem::buffer_t;

void pen(short int, short int, short int, short int);
void clear();

void rect(long int, long int, long int, long int);
void frect(long int, long int, long int, long int);

void text(rust::Str, int32_t, int32_t);
uint32_t text_width(rust::Str);

std::unique_ptr<buffer_t> buffer(uint32_t, uint32_t, rust::Slice<const uint8_t>);
void blit(const buffer_t&, int32_t, int32_t, int32_t, int32_t, int32_t, int32_t);

void set_blend_alpha_enabled(bool);

bool button(uint32_t);
bool pressed(uint32_t);

uint32_t time_ms();
uint32_t time_us();

void wait_vsync();
bool is_flipping();
void flip();

void update_gpio();
