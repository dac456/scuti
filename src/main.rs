#![crate_name = "scuti"]

extern crate glutin;
extern crate gl;

use gl::types::*;

fn main() {
    let window = glutin::Window::new().unwrap();


    unsafe {
        window.make_current();
        gl::load_with(|s| window.get_proc_address(s) as *const _);
    }

    for event in window.wait_events() {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }

        window.swap_buffers();
    }
}
