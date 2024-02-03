extern crate gl;
extern crate glfw;
extern crate skia_safe;

use gl::types::*;
use glfw::{Action, Context, Glfw, Key, WindowEvent, WindowHint};
use skia_safe::Color;

struct WindowContext {
    glfw: Glfw,
    window: glfw::PWindow,
    event_receiver: glfw::GlfwReceiver<(f64, WindowEvent)>,

    samples: u32,
    stencil_bits: u32,
}

/// Create a window and GL context with GLFW
fn create_window() -> WindowContext {

    // Window/GL Context Settings
    let num_samples = 0;
    let stencil_bits = 8;
    let x_size = 1024;
    let y_size = 1024;
    let title = "Hello, Skia!";

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.default_window_hints();
    glfw.window_hint(WindowHint::Samples(Some(num_samples)));
    glfw.window_hint(WindowHint::StencilBits(Some(stencil_bits)));
    glfw.window_hint(WindowHint::Resizable(false));

    let (mut window, events) = glfw.create_window(x_size, y_size, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    WindowContext {
        glfw,
        window,
        event_receiver: events,
        samples: num_samples,
        stencil_bits,
    }
}

fn create_skia_context() -> skia_safe::gpu::DirectContext {
    let interface = skia_safe::gpu::gl::Interface::new_native().unwrap();
    skia_safe::gpu::DirectContext::new_gl(Some(interface), None).unwrap()
}

// Create a Skia surface to cover a window
fn create_skia_surface(window_context: &WindowContext, context: &mut skia_safe::gpu::RecordingContext) -> skia_safe::Surface {
    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        skia_safe::gpu::gl::FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        }
    };

    let backend_render_target = skia_safe::gpu::backend_render_targets::make_gl(
        window_context.window.get_size(),
        window_context.samples as usize,
        window_context.stencil_bits as usize,
        fb_info,
    );

    skia_safe::gpu::surfaces::wrap_backend_render_target(
        context,
        &backend_render_target,
        skia_safe::gpu::SurfaceOrigin::BottomLeft,
        skia_safe::ColorType::RGBA8888,
        None,
        None,
    ).unwrap()
}
    

fn main() {
    let mut window_context = create_window();
    let mut skia_ctx = create_skia_context();
    let mut surface = create_skia_surface(&window_context, &mut skia_ctx);

    while !window_context.window.should_close() {
        window_context.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window_context.event_receiver) {
            handle_window_event(&mut window_context.window, event);
        }

        let canvas = surface.canvas();
        canvas.clear(Color::MAGENTA);

        skia_ctx.flush_and_submit();
        window_context.window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}