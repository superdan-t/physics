use std::vec::Vec;

use crate::model;
use crate::renderer;

/// An object in the 2D simulation. All objects are circles for now.
pub struct Object {
    pub graphics_model: model::Circle,
    pub id: u32,
}

/// The root controller of the 2D simulation
pub struct Simulation<DrawStrategy>
where
    DrawStrategy: renderer::Renderer,
{
    objects: Vec<Object>,
    object_counter: u32,
    draw_strategy: DrawStrategy,
}

impl<DrawStrategy> Simulation<DrawStrategy>
where
    DrawStrategy: renderer::Renderer,
{
    /// Create a new simulation
    pub fn new(draw_strategy: DrawStrategy) -> Simulation<DrawStrategy> {
        Simulation {
            objects: Vec::new(),
            object_counter: 0,
            draw_strategy,
        }
    }

    /// Add an object to the simulation
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);

        self.objects.last_mut().unwrap().id = self.object_counter;
        self.object_counter += 1;
    }

    /// Remove an object from the simulation
    pub fn remove_object(&mut self, id: u32) {
        self.objects.retain(|object| object.id != id);
    }

    /// Draw all elements in the simulation
    pub fn draw(&mut self) {
        self.draw_strategy.clear();
        for object in &self.objects {
            self.draw_strategy.draw_circle(&object.graphics_model);
        }
    }
}
