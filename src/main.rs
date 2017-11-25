#![crate_name = "scuti"]

extern crate glfw;
extern crate gl;

use std::path::{Path, PathBuf};
use std::thread::Builder;
use std::sync::mpsc::{channel, Receiver};

use glfw::{Action, Context, Key};

mod core;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(640, 480, "Scuti", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.set_key_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let render_context = window.render_context();


    let (send, recv) = channel();

    let render_task = Builder::new().name("render_task".to_string());
//    let render_task_done = render_task.spawn(move || { render(render_context, recv)});

    let engine = core::engine::Engine::new(PathBuf::from("./media/"));
    engine.start();


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        }


        window.swap_buffers();
    }

    send.send(()).ok().expect("Failed signal to render thread.");

//    let _ = render_task_done;
}

fn render(mut ctx: glfw::RenderContext, finish: Receiver<()>) {
    ctx.make_current();

    loop {
        if finish.try_recv() == Ok(()) {
            break;
        }

        unsafe {
            gl::Viewport(0, 0, 640, 480);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        }


        ctx.swap_buffers();
    }

    glfw::make_context_current(None);
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}