use std::{
    collections::BTreeMap, ops::{
        Deref,
        DerefMut
    }
};
use super::{
    component_table::ComponentTable,
    system::System,
};

/// A world is a collection of entities, components and systems.
/// Entities represents objects, components are pieces of data on those objects.
/// we can also add singleton : components that are not attached to entities.
/// Systems can iterate over components, and change such data.
#[derive(Default)] // todo : clone, debug
pub struct World {
    // can be considerer to be a world
    // all the components on the entities
    components: ComponentTable,
    // all the systems, ids being order of execution
    systems: BTreeMap<u64, System>,
}

impl Deref for World {
    type Target = ComponentTable;
    /// get access to the inner components table of the world.
    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl DerefMut for World {
    /// get a mutable access to the inner components of the world.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.components
    }
}

impl World {
    /// Register a system in the world. The index gives the order of update of all the system, starting from 0.
    #[inline]
    pub fn register_system(&mut self, system: System, index: u64) -> Option<System> {
        self.systems.insert(index, system)
    }

    /// Get a reference to a registered system by id.
    pub fn get_system(&self, index: u64) -> Option<&System> {
        self.systems.get(&index)
    }

    /// Get a mutable reference to a regsistered system by id.
    pub fn get_system_mut(&mut self, index: u64) -> Option<&mut System> {
        self.systems.get_mut(&index)
    }

    /// Get a reference to a system, keeping a reference to the component table.
    pub fn get_system_and_world(&self, index: u64) -> Option<(&System, &ComponentTable)> {
        self.systems.get(&index).map(|system| (system, &self.components))
    }

    /// Get a mutable reference to a system, keeping a mutable reference to the component table.
    pub fn get_system_and_world_mut(&mut self, index: u64) -> Option<(&mut System, &mut ComponentTable)> {
        self.systems.get_mut(&index).map(|system| (system, &mut self.components))
    }

    /// removes a system from the world.
    pub fn remove_system(&mut self, index: u64) -> Option<System> {
        self.systems.remove(&index)
    }

    /// Call an update on every registered systems.
    pub fn update(&mut self, delta: f32) {
        // update every system in order
        for (_id, system) in self.systems.iter_mut() {
            system.update(&mut self.components, delta);
        }
    }

}

