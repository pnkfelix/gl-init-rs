# glutin -  OpenGL, UTilities and INput
[![Gitter](https://badges.gitter.im/Join Chat.svg)](https://gitter.im/tomaka/glutin?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Alternative to GLFW in pure Rust.

[![Build Status](https://travis-ci.org/tomaka/glutin.png?branch=master)](https://travis-ci.org/tomaka/glutin)

## Try it!

```bash
git clone https://github.com/tomaka/glutin
cd glutin
cargo test
./target/test/window    # or target\test\window.exe
```

## Usage

```rust
extern crate glutin;
extern crate libc;
extern crate gl;

fn main() {
    let window = glutin::Window::new().unwrap();

    unsafe { window.make_current() };

    gl::load_with(|symbol| window.get_proc_address(symbol));

    gl::ClearColor(0.0, 1.0, 0.0, 1.0);

    while !window.is_closed() {
        window.wait_events();

        gl::Clear(gl::COLOR_BUFFER_BIT);

        window.swap_buffers();
    }
}
```

## Platform-specific notes

### Android

 - To compile the examples for android, initialize the submodules, go to `deps/apk-builder/apk-builder` and run `cargo build`, then go back to `gl-init` and call `ANDROID_HOME=/path/to/sdk NDK_HOME=/path/to/ndk NDK_STANDALONE=/path/to/standalone cargo test --no-run --target=arm-linux-androideabi`
 - Events are not implemented
 - Headless rendering doesn't work

### Emscripten

 - Work will start when Emscripten gets updated to LLVM 3.5 (which should happen soon)

### OS/X

 - Events are not implemented

### Win32

 - Pixel formats are not implemented
 - If you don't have MinGW installed, you will need to provide `libgdi32.a` and `libopengl32.a` ; you can put them in `C:\Users\you\.rust`
 - If you don't have `make` in your PATH, you can pass `--no-default-features --features "window"` when compiling ([see also](http://crates.io/manifest.html#the-[features]-section))

### X11

 - Some input events are not implemented
 - Pixel formats not implemented
 - The implementation probably needs a cleanup
