extern crate init = "gl-init-rs";
extern crate libc;
extern crate gl;

fn main() {
    let window1 = init::Window::new().unwrap();
    let window2 = init::Window::new().unwrap();

    spawn(proc() {
        run(window1, (0.0, 1.0, 0.0, 1.0));
    });

    spawn(proc() {
        run(window2, (0.0, 0.0, 1.0, 1.0));
    });
}

fn run(window: init::Window, color: (f32, f32, f32, f32)) {
    unsafe { window.make_current() };

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const libc::c_void);

    gl::ClearColor(color.val0(), color.val1(), color.val2(), color.val3());

    while !window.is_closed() {
       window.wait_events().collect::<Vec<init::Event>>();

        gl::Clear(gl::COLOR_BUFFER_BIT);

        window.swap_buffers();
    }
}