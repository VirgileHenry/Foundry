use crate::ecs::component_table::ComponentTable;

pub enum UpdateFrequency {
    PerFrame, // update once every frame
    Fixed(f32), // updates at a fixed rate
}

pub struct System {
    system: Box<dyn Updatable>, // the system struct implementing updatable
    frequency: UpdateFrequency,
    timer: f32,
}

impl System {
    pub fn new(system: Box<dyn Updatable>, update_frequency: UpdateFrequency) -> System {
        return System { system: system, frequency: update_frequency, timer: 0.0 };
    }

    pub fn update(&mut self, components: &ComponentTable, delta: f32) {
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
}

pub trait Updatable {
    fn update(&mut self, components: &ComponentTable, delta: f32);
}

