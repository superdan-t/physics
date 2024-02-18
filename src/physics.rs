//! Physics engine

use std::time::Duration;

#[derive(Default)]
pub struct Motion {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
}

/// A physics circle primitive
pub struct Circle {
    pub origin: (f32, f32),
    pub radius: f32,
}

/// A unique identifier for a body in the physics engine
///
/// Each body in the physics engine has a unique numeric ID. The wrapper type prevents accidental mixing of IDs from different systems or inadvertent arithmetic operations.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(usize);

/// One physical body in the physics simulation
///
/// Bodies in the simulation represent physical objects that can move and collide. Typically a game object will have a graphics model and a physics body.
pub struct Body {
    pub id: BodyId,
    pub circle: Circle,
    pub motion: Motion,
}

/// The root of the physics engine
///
/// The physics engine updates object states based on motion and collisions.
pub struct PhysicsEngine {
    objects: Vec<Body>,
}

impl PhysicsEngine {
    /// Create a new physics engine
    pub fn new() -> PhysicsEngine {
        PhysicsEngine {
            objects: Vec::new(),
        }
    }

    pub fn get_object(&self, id: BodyId) -> Option<&Body> {
        self.objects.get(id.0)
    }

    pub fn get_object_mut(&mut self, id: BodyId) -> Option<&mut Body> {
        self.objects.get_mut(id.0)
    }

    /// Add a new object to the physics engine
    pub fn add_object(&mut self, circle: Circle) -> &mut Body {
        self.objects.push(Body {
            id: BodyId(self.objects.len()),
            circle,
            motion: Motion::default(),
        });
        self.objects.last_mut().unwrap()
    }

    /// Update the physics engine state
    pub fn update(&mut self, dt: Duration) {
        for object in self.objects.iter_mut() {
            object.motion.position.0 += object.motion.velocity.0 * dt.as_secs_f32();
            object.motion.position.1 += object.motion.velocity.1 * dt.as_secs_f32();
        }
    }
}
