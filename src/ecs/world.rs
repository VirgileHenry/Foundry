use crate::utils::collections::packed_array::PackedArray;

use super::{
    component_table::ComponentTable,
    system::System,
    entity::{Entity}
};



pub struct World {
    // can be considerer to be a world
    // all the components on the entities
    pub components: ComponentTable, // todo : private
    // all the systems, ids being order of execution
    systems: PackedArray<System>,
}

impl World {
    pub fn new() -> World {
        return World {  
            components: ComponentTable::new(),
            systems: PackedArray::new(),
        };
    }

    #[inline]
    pub fn create_entity(&mut self) -> Entity {
        self.components.create_entity()
    }

    #[inline]
    pub fn destroy_entity(&mut self, entity: Entity) {
        self.components.destroy_entity(entity);
    }

    #[inline]
    pub fn create_entities(&mut self, count: usize) -> Vec<Entity> {
        self.components.create_entities(count)
    }

    #[inline]
    pub fn set_entity_active(&mut self, entity: &Entity, active: bool) {
        self.components.set_entity_active(entity, active);
    }
    
    #[inline]
    pub fn add_component<C: 'static>(&mut self, entity: &Entity, component: C) -> Option<C> {
        return self.components.add_component(entity, component);
    }

    #[inline]
    pub fn get_component<C: 'static>(&mut self, entity: &Entity) -> Option<&C> {
        return self.components.get_component::<C>(entity);
    }

    #[inline]
    pub fn get_component_mut<C: 'static>(&mut self, entity: &Entity) -> Option<&mut C> {
        return self.components.get_component_mut::<C>(entity);
    }

    #[inline]
    pub fn remove_component<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        return self.components.remove_component::<C>(entity);
    }

    #[inline]
    pub fn register_system(&mut self, system: System, index: usize) {
        self.systems.insert(system, index);
    }

    pub fn update(&mut self, delta: f32) {
        // update every system in order
        for system in self.systems.iter_mut() {
            system.elem.update(&mut self.components, delta);
        }
    }
}

