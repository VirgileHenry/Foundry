extern crate anymap;
use crate::{ecs::{
    entity::Entity,
    component_array::ComponentArray,
}, utils::collections::packed_array::IndexedElem};

pub struct ComponentTable {
    components: anymap::Map,
    active_entities: Vec<u8>,
    entity_count: usize,
}

impl ComponentTable {
    pub fn new() -> ComponentTable {
        return ComponentTable {
            components: anymap::Map::new(),
            active_entities: vec!(),
            entity_count: 0,
        };
    }

    pub fn create_entity(&mut self) -> Entity {
        let result = Entity {
            id: self.entity_count
        };
        if self.active_entities.len() * 8 <= self.entity_count {
            self.active_entities.push(0b1111_1111); // u8 max values so all 1s
        }
        self.entity_count += 1;
        result
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        // this is bad, we move the entity and deactivate it.
        // need to find a good way to remove all it's components to clean memory
        self.set_entity_active(&entity, false);
    }

    pub fn create_entities(&mut self, count: usize) -> Vec<Entity> {
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            result.push(Entity { id: self.entity_count + i });
        }
        if self.active_entities.len() * 8 <= self.entity_count + count - 1 {
            let mut vec: Vec<u8> = vec!(0b1111_1111; (self.entity_count + count - self.active_entities.len() * 8) / 8);
            self.active_entities.append(&mut vec);
        }
        self.entity_count += count;
        result
    }

    pub fn set_entity_active(&mut self, entity: &Entity, active: bool) {
        match self.active_entities.get_mut(entity.id / 8) {
            Some(pack) => {
                if active {
                    // set the bit corresponding to the entity to 1
                    *pack |= 1 << (entity.id % 8);
                }
                else {
                    // set the bit corresponding to the entity to 0
                    *pack &= !(1 << (entity.id % 8));
                }
            }
            None => {} // should never happen, but in that case the given entity is not valid
        }
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

    pub fn get_active_entities(&self) -> &Vec<u8> {
        return &self.active_entities;
    }

    pub fn remove_component<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        return match self.components.get_mut::<ComponentArray<C>>() {
            None => None,
            Some(comp_arr) => comp_arr.remove_component(entity.id),
        }
    }

}

// macros to create entities with any number of components
#[macro_export]
macro_rules! create_entity {
    ($comp_table:expr) => { ComponentTable::create_entity(&mut $comp_table) };
    ($comp_table:expr; $($comp:expr),*) => { {
        let result_entity = ComponentTable::create_entity(&mut $comp_table);
        $(
            $comp_table.add_comp_to_last(&result_entity, $comp);
        )*
        result_entity
    } };
}

#[macro_export]
macro_rules! create_entities {
    ($comp_table:expr; $amount:expr, $($generators:expr),*) => {
        {
            let result_entities = ComponentTable::create_entities(&mut $comp_table, $amount);
            let start_index = match result_entities.get(0) {Some(entity) => entity.id, None => 0};
            $(
                let mut comp_vec = Vec::with_capacity($amount);
                for i in 0..$amount {
                    comp_vec.push($generators(i));
                }
                $comp_table.add_comps_to_last(start_index, comp_vec);
            )*
            result_entities
        }
    };
}