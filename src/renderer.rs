//! Renders 2D models to a surface

extern crate skia_safe;

use crate::model::Circle;

/// A renderer that can draw 2D models
pub trait Renderer {
    // Procedures

    /// Selects the region of the physics simulation to draw and fit it to the surface
    fn set_physics_region(&mut self, p1: (f32, f32), p2: (f32, f32));

    /// Clear the surface
    fn clear(&mut self);

    // Primitive shapes

    fn draw_circle(&mut self, circle: &Circle);
}

/// A draw strategy that uses Skia to draw to a surface
pub struct SkiaRenderer<'a> {
    pub surface: &'a mut skia_safe::Surface,
}

impl Renderer for SkiaRenderer<'_> {
    fn set_physics_region(&mut self, p1: (f32, f32), p2: (f32, f32)) {
        // Get the surface dimensions as f32
        let surface_width_f = self.surface.width() as f32;
        let surface_height_f = self.surface.height() as f32;

        let canvas = self.surface.canvas();
        canvas.reset_matrix();

        // Translate the canvas to use the origin of the physics region
        canvas.translate((-p1.0, -p1.1));

        // Scale the desired region to the surface dimensions
        canvas.scale((
            surface_width_f / (p2.0 - p1.0),
            surface_height_f / (p2.1 - p1.1),
        ));

        // Flip the canvas vertically to match the physics coordinate system
        canvas.scale((1.0, -1.0));
        canvas.translate((0.0, -(p2.1 - p1.1)));

        // Set the clip region to the desired region
        canvas.clip_rect(
            skia_safe::Rect::from_wh(surface_width_f, surface_height_f),
            skia_safe::ClipOp::Intersect,
            true,
        );
    }

    fn clear(&mut self) {
        let canvas = self.surface.canvas();
        canvas.clear(skia_safe::Color::BLACK);
    }

    fn draw_circle(&mut self, circle: &Circle) {
        let canvas = self.surface.canvas();
        let paint = skia_safe::Paint::new(skia_safe::Color4f::new(1.0, 1.0, 1.0, 1.0), None);
        canvas.draw_circle(circle.origin, circle.radius, &paint);
    }
}

impl SkiaRenderer<'_> {
    pub fn new(surface: &mut skia_safe::Surface) -> SkiaRenderer {
        SkiaRenderer { surface }
    }
}
