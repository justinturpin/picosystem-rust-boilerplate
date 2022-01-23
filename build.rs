// build.rs

use std::str::FromStr;
use std::env;
use std::path::PathBuf;

fn cargo_relative_path(p: &str) -> PathBuf {
    let mut path = PathBuf::from_str(
        env::var("CARGO_MANIFEST_DIR").unwrap().as_str()
    ).unwrap();

    path.push(p);

    path
}

fn pico_sdk_relative_path(p: &str) -> PathBuf {
    let mut path = PathBuf::from_str(
        env::var("PICO_SDK_PATH").expect("PICO_SDK_PATH must be specified.").as_str()
    ).unwrap();

    path.push(p);

    path
}

fn picosystem_relative_path(p: &str) -> PathBuf {
    let mut path = PathBuf::from_str(
        env::var("PICOSYSTEM_DIR").expect("PICOSYSTEM_DIR must be specified.").as_str()
    ).unwrap();

    path.push(p);

    path
}

fn main() {
    let picosystem_include_path = picosystem_relative_path("libraries");

    cxx_build::CFG.exported_header_dirs.push(&picosystem_include_path);

    let generated_include_path = cargo_relative_path("generated/include");

    cxx_build::CFG.exported_header_dirs.push(&generated_include_path);

    let pico_common_paths = [
        "pico_base",
        "pico_bit_ops",
        "pico_binary_info",
        "pico_divider",
        "pico_sync",
        "pico_time",
        "pico_util",
        "pico_stdlib",
    ];

    for path in pico_common_paths {
        let mut full_path = PathBuf::from_str(
            env::var("PICO_SDK_PATH").expect("PICO_SDK_PATH must be specified.").as_str()
        ).unwrap();

        full_path.push("src/common");
        full_path.push(path);
        full_path.push("include");

        cxx_build::CFG.exported_header_dirs.push(&full_path);
    }

    // Pico common and hardware paths

    let rp2_paths = [
        "src/host/hardware_gpio/include",
        "src/rp2040/hardware_regs/include",
        "src/rp2040/hardware_structs/include",
        "src/rp2_common/pico_platform/include",
        "src/rp2_common/pico_stdio/include",
        "src/rp2_common/hardware_timer/include",
        "src/rp2_common/hardware_base/include",
        "src/rp2_common/hardware_uart/include",
    ];

    for path in rp2_paths {
        let pathbuf = pico_sdk_relative_path(path);

        cxx_build::CFG.exported_header_dirs.push(&pathbuf);
    }

    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .file("src/binding.cpp")
        .flag_if_supported("-std=c++14")
        .compile("picosystem-game-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/binding.cpp");
    println!("cargo:rerun-if-changed=include/binding.h");
    println!("cargo:rerun-if-changed=generated/include/config_autogen.h");
}
