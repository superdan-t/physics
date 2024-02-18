//! Contains root components of the physics simulator including the controller, objects, and inputs.

use std::default::Default;
use std::time::Duration;
use std::vec::Vec;

use crate::model::Primitive;
use crate::physics::{BodyId, Circle, PhysicsEngine};
use crate::renderer;

/// An object in the 2D simulation
pub struct Object {
    pub graphics_model: Primitive,
    pub physics_body: BodyId,
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

    pub physics: PhysicsEngine,

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
            physics: PhysicsEngine::new(),
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

            // The zoom speed is how much the view region should change per second. See the exponential function below.
            let z_speed = self.inputs.view_region_zoom_speed
                * delta_time.as_secs_f32()
                * self.inputs.view_region_zoom_speed_multiplier;

            // Convert the zoom speed to a scale factor using an exponential function with a base of 2
            // With a base of 2, this function causes the zoom to double with each positive unit of zoom speed and halve with each unit of negative zoom speed
            // Then, take the reciprocal of the scale factor to determine how much each dimension changes (positive zoom -> smaller region, ...)
            let delta_size = 1.0 / 2_f32.powf(z_speed);

            // Calculate the center of the view region
            let center = ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0);

            // Scale the view region around the center
            p1.0 = center.0 + (p1.0 - center.0) * delta_size; // x1
            p1.1 = center.1 + (p1.1 - center.1) * delta_size; // y1

            p2.0 = center.0 + (p2.0 - center.0) * delta_size; // x2
            p2.1 = center.1 + (p2.1 - center.1) * delta_size; // y2

            self.renderer.set_physics_region(p1, p2);
        }

        self.physics.update(delta_time);
    }

    pub fn add_object_with_model_at_pos(
        &mut self,
        model: Primitive,
        position: (f32, f32),
    ) -> &mut Object {
        // Create a physics model for circles only for now
        let body = match &model {
            Primitive::Circle(circle) => self.physics.add_object(Circle {
                origin: (0.0, 0.0),
                radius: circle.radius,
            }),
            Primitive::Rectangle(rectangle) => {
                // Create the largest possible circle that fits inside the rectangle
                // The origin of the rectangle model is the bottom-left corner
                self.physics.add_object(Circle {
                    origin: (
                        rectangle.origin.0 + rectangle.dimensions.0 / 2.0,
                        rectangle.origin.1 + rectangle.dimensions.1 / 2.0,
                    ),
                    radius: (rectangle.dimensions.0 / 2.0).min(rectangle.dimensions.1 / 2.0),
                })
            }
        };

        body.motion.position = position;

        self.objects.push(Object {
            graphics_model: model,
            physics_body: body.id,
            id: self.object_uid_counter,
        });
        self.object_uid_counter += 1;

        self.objects.last_mut().unwrap()
    }

    pub fn add_object_with_model(&mut self, model: Primitive) -> &mut Object {
        self.add_object_with_model_at_pos(model, (0.0, 0.0))
    }

    /// Remove an object from the simulation
    pub fn remove_object(&mut self, id: u32) {
        self.objects.retain(|object| object.id != id);
    }

    /// Draw all elements in the simulation
    pub fn draw_all(&mut self) {
        for object in &self.objects {
            self.renderer.draw_primitive_with_motion(
                &object.graphics_model,
                &self.physics.get_object(object.physics_body).unwrap().motion,
            );
        }
    }

    /// Complete all steps to render a new frame, including clearing, drawing, and submitting
    pub fn next_frame(&mut self) {
        self.renderer.begin_new_frame();
        self.draw_all();
        self.renderer.end_frame();
    }
}
