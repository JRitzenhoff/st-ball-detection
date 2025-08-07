fn main() {
    // According to ChatGPT --

    // Tells the Rust compiler to pass --nmagic to the linker
    //  Effect: disables page alignment of sections, allowing for tighter memory layout
    println!("cargo:rustc-link-arg-bins=--nmagic");

    // Tells the linker to use the custom linker script named link.x
    // The linker script points to a memory layout file named memory.x
    // The memory.x file is exported by the `embassy-stm32` crate (as defined in the cargo.toml)
    // This crate calls the `cortex-m-rt` crate under the hood
    //  See: https://docs.rs/cortex-m-rt/latest/cortex_m_rt/
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    // Adds a custom logging file to the executable
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
