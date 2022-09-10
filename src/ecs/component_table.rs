extern crate anymap;
use crate::{ecs::{
    entity::Entity,
    component_array::ComponentArray,
}, utils::collections::packed_array::IndexedElem};

pub struct ComponentTable {
    components: anymap::Map,
    last_entity_id: usize,
}

impl ComponentTable {
    pub fn new() -> ComponentTable {
        return ComponentTable {
            components: anymap::Map::new(),
            last_entity_id: 1,
        };
    }

    pub fn create_entity(&mut self) -> Entity {
        let result = Entity {
            id: self.last_entity_id
        };
        self.last_entity_id += 1;
        result
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        todo!();
    }

    pub fn create_entities(&mut self, count: usize) -> Vec<Entity> {
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            result.push(Entity { id: self.last_entity_id + i });
        }
        self.last_entity_id += count;
        result
    }

    pub fn add_component<C: 'static>(&mut self, entity: &Entity, component: C) -> Option<C> {
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => {
                // case where the component array exist
                return components.insert_component(component, entity.id);
            },
            None => {
                // create the component array from the new element
                self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_elem(component, entity.id));
                return None;
            },
        };
    }

    pub fn add_comp_to_last<C: 'static>(&mut self, entity: &Entity, component: C) {
        // this may only be used if we are ensured the entity is the last one, and it does not have this component yet
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => components.append_component(component, entity.id),
            None => { self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_elem(component, entity.id)); },
        };
    }

    pub fn add_comps_to_last<C: 'static>(&mut self, start_index: usize, component_vec: Vec<C>) {
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => components.append_components(component_vec, start_index),
            None => { self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_vec(component_vec, start_index));},
        };
    }

    pub fn get_component_map(&self) -> &anymap::Map {
        return &self.components;
    }

    pub fn get_component_map_mut(&mut self) -> &mut anymap::Map {
        return &mut self.components;
    }

    pub fn get_component_array<C: 'static>(&self) -> Option<&Vec<IndexedElem<C>>> {
        return match self.components.get::<ComponentArray<C>>() {
            Some(comp_arr) => Some(comp_arr.get_array()),
            None => None,
        }
    }

    pub fn get_component_array_mut<C: 'static>(&self) -> Option<&mut Vec<IndexedElem<C>>> {
        return match self.components.get::<ComponentArray<C>>() {
            Some(comp_arr) => Some(comp_arr.unsafe_get_array_mut()),
            None => None,
        }
    }

    pub fn get_component<C: 'static>(&self, entity: &Entity) -> Option<&C> {
        return match self.components.get::<ComponentArray<C>>() {
            None => None,
            Some(comp_arr) => comp_arr.get_component(entity.id),
        }
    }

    pub fn get_component_mut<C: 'static>(&mut self, entity: &Entity) -> Option<&mut C> {
        return match self.components.get_mut::<ComponentArray<C>>() {
            None => None,
            Some(comp_arr) => comp_arr.get_component_mut(entity.id),
        }
    }

    pub fn remove_component<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        return match self.components.get_mut::<ComponentArray<C>>() {
            None => None,
            Some(comp_arr) => comp_arr.remove_component(entity.id),
        }
    }

}
