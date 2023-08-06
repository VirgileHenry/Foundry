use crate::ecs::component_table::ComponentTable;
use std::fmt::Debug;

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

impl System {
    /// Creates a new system from any struct implementing the ```Updatable``` trait.
    pub fn new<T: Updatable + 'static>(system: T, update_frequency: UpdateFrequency) -> System {
        return System { system: Box::new(system), frequency: update_frequency, timer: 0.0 };
    }

    /// Update the system.
    /// This may not cause an update on the actual system if delta is smaller than the fixed time step update frequency.
    pub fn update(&mut self, components: &mut ComponentTable, delta: f32) {
        match self.frequency {
            UpdateFrequency::PerFrame => self.system.update(components, delta),
            UpdateFrequency::Fixed(freq) => {
                self.timer += delta;
                while self.timer >= freq {
                    self.system.update(components, freq);
                    self.timer -= freq;
                }
            }
        }
    }

    /// Try to downcast the inner updatable to a concrete type.
    pub fn try_get_updatable<T: Updatable + 'static>(&self) -> Option<&T> {
        return self.system.as_any().downcast_ref::<T>();
    }

    /// Try to downcast the inner updatable to a concrete type, as mutable.
    pub fn try_get_updatable_mut<T: Updatable + 'static>(&mut self) -> Option<&mut T> {
        return self.system.as_any_mut().downcast_mut::<T>();
    }
}

/// Trait that allow any struct to be used as a system.
pub trait Updatable: AsAny {
    fn update(&mut self, components: &mut ComponentTable, delta: f32);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/*
impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
*/