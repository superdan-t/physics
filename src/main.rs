extern crate gl;
extern crate glfw;
extern crate skia_safe;

pub mod model;
pub mod renderer;
pub mod simulation;

use glfw::{Action, Context, Glfw, Key, WindowEvent, WindowHint};
use skia_safe::Color;

use model::primitive::*;
use renderer::Renderer;
use renderer::SkiaRenderer;
use simulation::Simulation;

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

    let (mut window, events) = glfw
        .create_window(x_size, y_size, title, glfw::WindowMode::Windowed)
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

fn main() {
    let mut window_context = create_window();

    let context_properties = renderer::SurfaceProperties {
        dimensions: window_context.window.get_size(),
        num_samples: window_context.samples,
        stencil_bits: window_context.stencil_bits,
    };
    let mut renderer = SkiaRenderer::new(&context_properties);

    renderer.set_physics_region((0.0, 0.0), (100.0, 100.0));

    let mut simulation = Simulation::new();

    // Draw a background rectangle
    simulation.add_object_with_model(
        Rectangle {
            origin: (0.0, 0.0),
            dimensions: (100.0, 100.0),
            color: Color::from_rgb(8, 0, 22),
        }
        .into(),
    );

    // Add a fun circle
    simulation.add_object_with_model(
        Circle {
            origin: (50.0, 25.0),
            radius: 2.0,
            color: Color::WHITE,
        }
        .into(),
    );

    while !window_context.window.should_close() {
        window_context.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window_context.event_receiver) {
            handle_window_event(&mut window_context.window, event);
        }
        renderer.begin_new_frame();

        simulation.draw_all(&mut renderer);

        renderer.end_frame();
        window_context.window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
