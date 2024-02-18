//! Defines 2D graphics models

pub mod primitive {

    use skia_safe::Color;

    use crate::physics::Motion;

    /// A circle with a center origin
    pub struct Circle {
        pub origin: (f32, f32),
        pub radius: f32,
        pub color: Color,
    }

    /// A rectangle with a top-left origin
    pub struct Rectangle {
        pub origin: (f32, f32),
        pub dimensions: (f32, f32),
        pub color: Color,
    }

    impl Circle {
        pub fn with_motion(&self, motion: &Motion) -> Circle {
            Circle {
                origin: (
                    self.origin.0 + motion.position.0,
                    self.origin.1 + motion.position.1,
                ),
                radius: self.radius,
                color: self.color,
            }
        }
    }

    impl Rectangle {
        pub fn with_motion(&self, motion: &Motion) -> Rectangle {
            Rectangle {
                origin: (
                    self.origin.0 + motion.position.0,
                    self.origin.1 + motion.position.1,
                ),
                dimensions: self.dimensions,
                color: self.color,
            }
        }
    }
}

use std::convert::From;

use primitive::*;

/// A 2D primitive model
///
/// This enum represents the 2D primitive models. More complex models can be created by combining these primitives.
pub enum Primitive {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl From<Circle> for Primitive {
    fn from(circle: Circle) -> Primitive {
        Primitive::Circle(circle)
    }
}

impl From<Rectangle> for Primitive {
    fn from(rectangle: Rectangle) -> Primitive {
        Primitive::Rectangle(rectangle)
    }
}
