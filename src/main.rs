extern crate gl;
extern crate glfw;
extern crate skia_safe;

pub mod model;
pub mod physics;
pub mod renderer;
pub mod simulation;

use std::time::Instant;

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
    let title = "Physics Simulation";

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

    let mut simulation = Simulation::new(renderer);

    simulation.inputs.view_region_scroll_speed_multiplier = 50.0;

    // Draw a background rectangle
    simulation.add_object_with_model(
        Rectangle {
            origin: (0.0, 0.0),
            dimensions: (100.0, 100.0),
            color: Color::from_rgb(8, 0, 22),
        }
        .into(),
    );

    // Add circles in an L shape to verify the canvas is flipped correctly
    let d1_body = simulation
        .add_object_with_model_at_pos(
            Circle {
                origin: (0.0, 0.0),
                radius: 2.0,
                color: Color::WHITE,
            }
            .into(),
            (25.0, 25.0),
        )
        .physics_body;
    simulation.add_object_with_model_at_pos(
        Circle {
            origin: (0.0, 0.0),
            radius: 2.0,
            color: Color::WHITE,
        }
        .into(),
        (25.0, 75.0),
    );
    simulation.add_object_with_model_at_pos(
        Circle {
            origin: (0.0, 0.0),
            radius: 2.0,
            color: Color::WHITE,
        }
        .into(),
        (50.0, 25.0),
    );

    simulation
        .physics
        .get_object_mut(d1_body)
        .unwrap()
        .dynamics
        .velocity = (0.0, 1.0);

    let mut last_frame_time = Instant::now();

    while !window_context.window.should_close() {
        window_context.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window_context.event_receiver) {
            handle_window_event(&mut window_context.window, event, &mut simulation);
        }

        let delta_time = last_frame_time.elapsed();
        last_frame_time = Instant::now();

        simulation.update(delta_time);

        simulation.next_frame();

        window_context.window.swap_buffers();
    }
}

fn handle_window_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    simulation: &mut Simulation<SkiaRenderer>,
) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

        // Zoom controls
        WindowEvent::Key(Key::Kp9, _, Action::Press, _) => {
            simulation.inputs.view_region_zoom_speed = 1.0
        }
        WindowEvent::Key(Key::Kp7, _, Action::Press, _) => {
            simulation.inputs.view_region_zoom_speed = -1.0
        }

        WindowEvent::Key(Key::Kp9, _, Action::Release, _) => {
            simulation.inputs.view_region_zoom_speed = 0.0
        }
        WindowEvent::Key(Key::Kp7, _, Action::Release, _) => {
            simulation.inputs.view_region_zoom_speed = 0.0
        }

        // Begin scrolling
        WindowEvent::Key(Key::Kp4, _, Action::Press, _) => {
            simulation.inputs.view_region_scroll_speed.0 = -1.0
        }
        WindowEvent::Key(Key::Kp6, _, Action::Press, _) => {
            simulation.inputs.view_region_scroll_speed.0 = 1.0
        }
        WindowEvent::Key(Key::Kp8, _, Action::Press, _) => {
            simulation.inputs.view_region_scroll_speed.1 = 1.0
        }
        WindowEvent::Key(Key::Kp2, _, Action::Press, _) => {
            simulation.inputs.view_region_scroll_speed.1 = -1.0
        }

        // End scrolling
        WindowEvent::Key(Key::Kp4, _, Action::Release, _) => {
            simulation.inputs.view_region_scroll_speed.0 = 0.0
        }
        WindowEvent::Key(Key::Kp6, _, Action::Release, _) => {
            simulation.inputs.view_region_scroll_speed.0 = 0.0
        }
        WindowEvent::Key(Key::Kp8, _, Action::Release, _) => {
            simulation.inputs.view_region_scroll_speed.1 = 0.0
        }
        WindowEvent::Key(Key::Kp2, _, Action::Release, _) => {
            simulation.inputs.view_region_scroll_speed.1 = 0.0
        }

        // Scroll controls

        // Reset the view
        WindowEvent::Key(Key::Kp5, _, Action::Release, _) => {
            simulation
                .renderer
                .set_physics_region((0.0, 0.0), (100.0, 100.0));
        }

        _ => {}
    }
}
