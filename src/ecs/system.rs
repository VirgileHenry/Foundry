use crate::ecs::component_table::ComponentTable;
use std::{any::Any, fmt::Debug};
use dyn_clone::{DynClone, clone_box};

/// Describes if a system should update every frame or on a fixed time step.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum UpdateFrequency {
    /// Update every frame.
    #[default]
    PerFrame,
    /// updates on a fixed time step.
    Fixed(f32),
}

/// A system is a struct wrapping any structure implementing the ```Updatable``` trait, allowing it to act on components.
pub struct System {
    /// the system struct implementing updatable
    system: Box<dyn Updatable>,
    /// The update frequency of this system
    frequency: UpdateFrequency,
    /// intern timer for fixed time step of update frequency
    timer: f32,
}

impl Clone for System {
    fn clone(&self) -> Self {
        System { 
            system: clone_box(self.system.as_ref()),
            frequency: self.frequency,
            timer: self.timer
        }
    }
}

impl System {
    /// Creates a new system from any struct implementing the ```Updatable``` trait.
    pub fn new(system: Box<dyn Updatable>, update_frequency: UpdateFrequency) -> System {
        return System { system: system, frequency: update_frequency, timer: 0.0 };
    }

    /// Update the system.
    /// This may not cause an update on the actual system if delta is smaller than the fixed time step update frequency.
    pub fn update(&mut self, components: &mut ComponentTable, delta: f32, user_data: &mut dyn Any) {
        match self.frequency {
            UpdateFrequency::PerFrame => self.system.update(components, delta, user_data),
            UpdateFrequency::Fixed(freq) => {
                self.timer += delta;
                while self.timer >= freq {
                    self.system.update(components, freq, user_data);
                    self.timer -= freq;
                }
            }
        }
    }
}

/// Trait that allow any struct to be used as a system.
pub trait Updatable: DynClone {
    /// update that will be called by the system manager.
    fn update(&mut self, components: &mut ComponentTable, delta: f32, user_data: &mut dyn Any);
}

