use std::vec::Vec;

use crate::model::Primitive;
use crate::renderer;

/// An object in the 2D simulation
pub struct Object {
    pub graphics_model: Primitive,
    pub id: u32,
}

/// The root controller of the 2D simulation
pub struct Simulation {
    /// A list of all objects in the simulation.
    ///
    /// Vector for now but as models get more complex we may want to avoid the overhead of dynamic resizing.
    objects: Vec<Object>,

    /// A counter for unique object IDs. Hopefully this will never overflow...
    object_uid_counter: u32,
}

impl Simulation {
    /// Create a new simulation
    pub fn new() -> Simulation {
        Simulation {
            objects: Vec::new(),
            object_uid_counter: 0,
        }
    }

    pub fn add_object_with_model(&mut self, model: Primitive) -> u32 {
        self.objects.push(Object {
            graphics_model: model,
            id: self.object_uid_counter,
        });
        self.object_uid_counter += 1;

        self.object_uid_counter - 1
    }

    /// Remove an object from the simulation
    pub fn remove_object(&mut self, id: u32) {
        self.objects.retain(|object| object.id != id);
    }

    /// Draw all elements in the simulation
    pub fn draw_all<Renderer: renderer::Renderer>(&mut self, renderer: &mut Renderer) {
        for object in &self.objects {
            renderer.draw_primitive(&object.graphics_model);
        }
    }
}
