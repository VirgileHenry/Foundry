use crate::utils::collections::packed_array::PackedArray;

use super::{
    component_table::ComponentTable,
    system::System,
    entity::{Entity}
};

/// world is the base of the ecs.
/// When creating an ecs, we create a world and use it's methods to manipulate it.
pub struct World {
    // can be considerer to be a world
    // all the components on the entities
    pub components: ComponentTable, // todo : private
    // all the systems, ids being order of execution
    systems: PackedArray<System>,
}

impl World {
    /// Create a new empty world.
    pub fn new() -> World {
        return World {  
            components: ComponentTable::new(),
            systems: PackedArray::new(),
        };
    }

    /// Create an empty entity in the world and returns it.
    #[inline]
    pub fn create_entity(&mut self) -> Entity {
        self.components.create_entity()
    }

    /// Remove an entity from the world.
    /// This is partially implemented, as it only deactivate the entity but does not destroy the components related to it.
    #[inline]
    pub fn destroy_entity(&mut self, entity: Entity) {
        self.components.destroy_entity(entity);
    }

    /// Creates multiple entities at once.
    /// It is more efficient than calling ```create_entity``` multiple times.
    #[inline]
    pub fn create_entities(&mut self, count: usize) -> Vec<Entity> {
        self.components.create_entities(count)
    }

    /// Set an entity as active or not.
    /// Inactive entities still exists, but are ignored by iterators over components and are not updated.
    #[inline]
    pub fn set_entity_active(&mut self, entity: &Entity, active: bool) {
        self.components.set_entity_active(entity, active);
    }
    
    /// Add a given component to an entity.
    /// If the entity already had a component of this type, replace it and return it.
    #[inline]
    pub fn add_component<C: 'static>(&mut self, entity: &Entity, component: C) -> Option<C> {
        return self.components.add_component(entity, component);
    }

    /// Get a reference to a component of the given type of an entity.
    #[inline]
    pub fn get_component<C: 'static>(&mut self, entity: &Entity) -> Option<&C> {
        return self.components.get_component::<C>(entity);
    }

    /// Get a mutable reference to a component of the given type of an entity.
    #[inline]
    pub fn get_component_mut<C: 'static>(&mut self, entity: &Entity) -> Option<&mut C> {
        return self.components.get_component_mut::<C>(entity);
    }

    /// Removes a component of the given type of an entity, and return it if there was any.
    #[inline]
    pub fn remove_component<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        return self.components.remove_component::<C>(entity);
    }

    /// Register a system in the world. The index gives the order of update of all the system, starting from 0.
    #[inline]
    pub fn register_system(&mut self, system: System, index: usize) {
        self.systems.insert(system, index);
    }

    pub fn get_system(&self, index: usize) -> Option<&System> {
        self.systems.get(index)
    }

    pub fn get_system_mut(&mut self, index: usize) -> Option<&mut System> {
        self.systems.get_mut(index)
    }

    /// Call an update on every registered systems.
    pub fn update(&mut self, delta: f32) {
        // update every system in order
        for system in self.systems.iter_mut() {
            system.elem.update(&mut self.components, delta);
        }
    }
}

