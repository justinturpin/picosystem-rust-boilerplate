#![no_std]
#![no_main]
#![feature(const_fn_fn_ptr_basics)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use alloc::vec::Vec;
use alloc::format;

mod game;
mod util;

// Credit to https://gist.github.com/huntc/ab9a505683647aac7bccd2df0fc75f9e
#[allow(non_camel_case_types)]
pub struct c_void;

pub struct ExternalHeap {
    allocator: extern "C" fn(u32) -> *mut c_void,
    deallocator: extern "C" fn(*mut c_void),
}

impl ExternalHeap {
    /// Initialize the static allocator with benign allocation/deallocation.
    /// This will always be the first call to make.
    pub const fn empty() -> ExternalHeap {
        ExternalHeap {
            allocator: ExternalHeap::noop_allocator,
            deallocator: ExternalHeap::noop_deallocator,
        }
    }

    /// Set up the external allocation/deallocation functions. This should be
    /// done prior to performing any allocations, otherwise you may find that
    /// panics occur given null allocations.
    pub fn init(
        &mut self,
        allocator: extern "C" fn(u32) -> *mut c_void,
        deallocator: extern "C" fn(*mut c_void),
    ) {
        self.allocator = allocator;
        self.deallocator = deallocator;
    }

    extern "C" fn noop_allocator(_size: u32) -> *mut c_void {
        0 as *mut c_void
    }

    extern "C" fn noop_deallocator(_ptr: *mut c_void) {}
}

unsafe impl GlobalAlloc for ExternalHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (self.allocator)(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        (self.deallocator)(ptr as *mut c_void)
    }
}

#[global_allocator]
static mut ALLOCATOR: ExternalHeap = ExternalHeap::empty();

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("picosystem-boilerplate-rs/include/binding.h");

        type buffer_t;

        fn pen(r: i16, g: i16, b: i16, a: i16);
        fn clear();
        fn rect(x: i32, y: i32, w: i32, h: i32);
        fn frect(x: i32, y: i32, w: i32, h: i32);

        fn text(message: &str, x: i32, y: i32);
        fn text_width(message: &str) -> u32;

        fn buffer(w: u32, h: u32, data: &[u8]) -> UniquePtr<buffer_t>;
        fn blit(buffer: &buffer_t, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32);

        fn set_blend_alpha_enabled(enabled: bool);

        fn button(b: u32) -> bool;
        fn pressed(b: u32) -> bool;

        fn time_ms() -> u32;
        fn time_us() -> u32;

        fn wait_vsync();
        fn is_flipping() -> bool;
        fn flip();

        fn update_gpio();
    }
}

#[repr(u32)]
pub enum Button {
    UP    = 23,
    DOWN  = 20,
    LEFT  = 22,
    RIGHT = 21,
    A     = 18,
    B     = 19,
    X     = 17,
    Y     = 16
}

#[macro_use]
mod macros {
    #[repr(C)] // guarantee 'bytes' comes after '_align'
    pub struct AlignedAs<Align, Bytes: ?Sized> {
        pub _align: [Align; 0],
        pub bytes: Bytes,
    }

    #[macro_export]
    macro_rules! include_bytes_align_as {
        ($align_ty:ty, $path:literal) => {
            {  // const block expression to encapsulate the static
                use $crate::macros::AlignedAs;

                // this assignment is made possible by CoerceUnsized
                static ALIGNED: &AlignedAs::<$align_ty, [u8]> = &AlignedAs {
                    _align: [],
                    bytes: *include_bytes!($path),
                };

                &ALIGNED.bytes
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn configure_heap(
    alloc: extern "C" fn(u32) -> *mut c_void,
    dealloc: extern "C" fn(*mut c_void)
) {
    unsafe { ALLOCATOR.init(alloc, dealloc) }
}

#[no_mangle]
pub extern "C" fn game_main() -> ! {
    let mut game = game::Game::new();

    loop {
        ffi::update_gpio();

        // --------------------------------------
        // game.update
        // --------------------------------------

        game.update(0);

        // --------------------------------------
        // wait to be done flipping
        // --------------------------------------

        while ffi::is_flipping() {}

        // --------------------------------------
        // game.draw
        // --------------------------------------

        game.draw();

        // --------------------------------------
        // finish
        // --------------------------------------

        ffi::wait_vsync();
        ffi::flip();
    }
}
