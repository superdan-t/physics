//! Renders 2D models to a surface

extern crate gl;
extern crate skia_safe;

use gl::types::*;
use skia_safe::gpu::{gl as skia_gl, DirectContext, RecordingContext};
use skia_safe::{gpu, Surface};

use crate::model::{primitive::*, Primitive};

/// A renderer that can draw 2D models
///
/// Renderers draw objects in a region of the physics simulation to a surface. All objects models consist of a set of primitive shapes.
pub trait Renderer {
    /// Selects the region of the physics simulation to draw and fit it to the surface
    fn set_physics_region(&mut self, p1: (f32, f32), p2: (f32, f32));

    fn get_physics_view_region(&self) -> ((f32, f32), (f32, f32));

    /// Resize the surface
    fn resize_surface(&mut self, dimensions: (i32, i32));

    /// Prepare to draw a new frame
    fn begin_new_frame(&mut self);

    /// Complete the current frame
    fn end_frame(&mut self);

    /*
     *	Primitive shapes
     */

    /// Draw a primitive shape
    fn draw_primitive(&mut self, primitive: &Primitive) {
        match primitive {
            Primitive::Circle(circle) => self.draw_circle(circle),
            Primitive::Rectangle(rectangle) => self.draw_rectangle(rectangle),
        }
    }

    /// Primitive shape
    fn draw_circle(&mut self, circle: &Circle);

    /// Primitive shape
    fn draw_rectangle(&mut self, rectangle: &Rectangle);
}

/// Properties of a GL surface
#[derive(Clone, Copy)]
pub struct SurfaceProperties {
    pub dimensions: (i32, i32),
    pub num_samples: u32,
    pub stencil_bits: u32,
}

/// A draw strategy that uses Skia to draw to a surface
pub struct SkiaRenderer {
    context: DirectContext,
    surface: Surface,

    surface_properties: SurfaceProperties,

    view_region: ((f32, f32), (f32, f32)),
}

impl Renderer for SkiaRenderer {
    fn set_physics_region(&mut self, p1: (f32, f32), p2: (f32, f32)) {
        // Get the surface dimensions as f32
        let surface_width_f = self.surface.width() as f32;
        let surface_height_f = self.surface.height() as f32;

        let canvas = self.surface.canvas();
        canvas.reset_matrix();

        // Flip the y-axis to match the physics coordinate system
        canvas.scale((1.0, -1.0));
        canvas.translate((0.0, -surface_height_f));

        // Scale the desired region to the surface dimensions
        canvas.scale((
            surface_width_f / (p2.0 - p1.0),
            surface_height_f / (p2.1 - p1.1),
        ));

        // Translate the canvas to use the origin of the physics region
        canvas.translate((-p1.0, -p1.1));

        self.view_region = (p1, p2);
    }

    fn get_physics_view_region(&self) -> ((f32, f32), (f32, f32)) {
        self.view_region
    }

    fn resize_surface(&mut self, dimensions: (i32, i32)) {
        self.surface_properties.dimensions = dimensions;
        self.surface = Self::create_surface(&mut self.context, &self.surface_properties);
    }

    fn begin_new_frame(&mut self) {
        self.surface.canvas().clear(skia_safe::Color::BLACK);
    }

    fn end_frame(&mut self) {
        self.context.flush_and_submit();
    }

    fn draw_circle(&mut self, circle: &Circle) {
        let canvas = self.surface.canvas();
        let mut paint = skia_safe::Paint::default();
        paint.set_color(circle.color);
        canvas.draw_circle(circle.origin, circle.radius, &paint);
    }

    fn draw_rectangle(&mut self, rectangle: &Rectangle) {
        let canvas = self.surface.canvas();
        let mut paint = skia_safe::Paint::default();
        paint.set_color(rectangle.color);
        canvas.draw_rect(
            skia_safe::Rect::from_xywh(
                rectangle.origin.0,
                rectangle.origin.1,
                rectangle.dimensions.0,
                rectangle.dimensions.1,
            ),
            &paint,
        );
    }
}

impl SkiaRenderer {
    pub fn new(properties: &SurfaceProperties) -> SkiaRenderer {
        let interface = skia_gl::Interface::new_native().unwrap();
        let mut context = DirectContext::new_gl(Some(interface), None).unwrap();

        let surface = Self::create_surface(&mut context, properties);
        let surface_dims = (surface.width() as f32, surface.height() as f32);

        let mut new_renderer = SkiaRenderer {
            context,
            surface,
            surface_properties: *properties,
            view_region: ((0.0, 0.0), (0.0, 0.0)),
        };

        new_renderer.set_physics_region((0.0, 0.0), surface_dims);

        new_renderer
    }

    /// Create a new surface
    fn create_surface(
        context: &mut RecordingContext,
        properties: &SurfaceProperties,
    ) -> skia_safe::Surface {
        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            skia_gl::FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let backend_render_target = gpu::backend_render_targets::make_gl(
            properties.dimensions,
            properties.num_samples as usize,
            properties.stencil_bits as usize,
            fb_info,
        );

        skia_safe::gpu::surfaces::wrap_backend_render_target(
            context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap()
    }
}
