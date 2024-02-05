//! Contains root components of the physics simulator including the controller, objects, and inputs.

use std::default::Default;
use std::time::Duration;
use std::vec::Vec;

use crate::model::Primitive;
use crate::renderer;

/// An object in the 2D simulation
pub struct Object {
    pub graphics_model: Primitive,
    pub id: u32,
}

/// Inputs to the simulation
pub struct Inputs {
    /// The speed to scroll the view region in physics units per second
    pub view_region_scroll_speed: (f32, f32),
    pub view_region_scroll_speed_multiplier: f32,
    pub view_region_zoom_speed: f32,
    pub view_region_zoom_speed_multiplier: f32,
}

impl Default for Inputs {
    fn default() -> Inputs {
        Inputs {
            view_region_scroll_speed: (0.0, 0.0),
            view_region_scroll_speed_multiplier: 1.0,
            view_region_zoom_speed: 0.0,
            view_region_zoom_speed_multiplier: 1.0,
        }
    }
}

/// The root controller of the 2D simulation
pub struct Simulation<Renderer: renderer::Renderer> {
    /// A list of all objects in the simulation.
    ///
    /// Vector for now but as models get more complex we may want to avoid the overhead of dynamic resizing.
    objects: Vec<Object>,

    /// A counter for unique object IDs. Hopefully this will never overflow...
    object_uid_counter: u32,

    dt_accum: f32,

    pub renderer: Renderer,

    pub inputs: Inputs,
}

impl<Renderer> Simulation<Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Create a new simulation
    pub fn new(renderer: Renderer) -> Simulation<Renderer> {
        Simulation {
            objects: Vec::new(),
            object_uid_counter: 0,
            dt_accum: 0.0,
            renderer,
            inputs: Inputs::default(),
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.dt_accum += delta_time.as_secs_f32();
        if self.inputs.view_region_scroll_speed.0 != 0.0
            || self.inputs.view_region_scroll_speed.1 != 0.0
            || self.inputs.view_region_zoom_speed != 0.0
        {
            let delta_x = self.inputs.view_region_scroll_speed.0
                * delta_time.as_secs_f32()
                * self.inputs.view_region_scroll_speed_multiplier;
            let delta_y = self.inputs.view_region_scroll_speed.1
                * delta_time.as_secs_f32()
                * self.inputs.view_region_scroll_speed_multiplier;

            let (mut p1, mut p2) = self.renderer.get_physics_view_region();

            p1.0 += delta_x;
            p1.1 += delta_y;

            p2.0 += delta_x;
            p2.1 += delta_y;

            self.renderer.set_physics_region(p1, p2);
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
    pub fn draw_all(&mut self) {
        for object in &self.objects {
            self.renderer.draw_primitive(&object.graphics_model);
        }
    }

    /// Complete all steps to render a new frame, including clearing, drawing, and submitting
    pub fn next_frame(&mut self) {
        self.renderer.begin_new_frame();
        self.draw_all();
        self.renderer.end_frame();
    }
}
