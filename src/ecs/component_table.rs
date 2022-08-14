extern crate anymap;
use std::collections::binary_heap::Iter;

use crate::utils::collections::packed_array::{PackedArray, IndexedElem, self};
use crate::ecs::{
    entity::Entity
};

pub struct ComponentTable {
    components: anymap::Map,
}

impl ComponentTable {
    pub fn new() -> ComponentTable {
        return ComponentTable {
            components: anymap::Map::new(),
        };
    }

    pub fn add_component<C: 'static>(&mut self, entity: &Entity, component: C) -> Option<C> {
        match self.components.get_mut::<PackedArray<C>>() {
            Some(packed_array) => {
                // case where the component array exist
                return packed_array.insert(component, entity.id);
            },
            None => {
                // create the component array from the new element
                self.components.insert::<PackedArray<C>>(PackedArray::<C>::new_with_elem(component, entity.id));
                return None;
            },
        };
    }

    pub fn add_comp_to_last<C: 'static>(&mut self, entity: &Entity, component: C) {
        // this may only be used if we are ensured the entity is the last one, and it does not have this component yet
        match self.components.get_mut::<PackedArray<C>>() {
            Some(packed_array) => packed_array.append(component, entity.id),
            None => { self.components.insert(PackedArray::new_with_elem(component, entity.id));} ,
        };
    }

    pub fn get_component<C: 'static>(&self, entity: &Entity) -> Option<&C> {
        return match self.components.get::<PackedArray<C>>() {
            None => None,
            Some(packed_array) => packed_array.get(entity.id),
        }
    }

    pub fn get_component_mut<C: 'static>(&mut self, entity: &Entity) -> Option<&mut C> {
        return match self.components.get_mut::<PackedArray<C>>() {
            None => None,
            Some(packed_array) => packed_array.get_mut(entity.id),
        }
    }

    pub fn remove_component<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        return match self.components.get_mut::<PackedArray<C>>() {
            None => None,
            Some(packed_array) => packed_array.remove(entity.id),
        }
    }

    pub fn iterate_over_1_component<'a, C: 'static>(&'a mut self) -> ComponentIterator1<C> {
        let result = ComponentIterator1::<C> {
            components_id: 0,
            iterator: match self.components.get_mut::<PackedArray<C>>() {
                None => None, // components is not in the table, so empty iterator
                Some(packed_array) => Some(packed_array.iter_mut()),
            },
            current_entity_id: 0,
            current_comp_index: 0,
        };
        return result;
    }
}


pub struct ComponentIterator1<C> {
    components_id: usize,
    current_entity_id: usize,
    current_comp_index: u32,
    iterator: dyn Iterator<Item = C>,
}

impl<C> Iterator for ComponentIterator1<C> {
    type Item = &mut C;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.iterator {
            Some(mut iterator) => {
                match iterator.next() {
                    Some(elem) => return Some(&mut elem.elem),
                    None => return None,
                }
            },
            None => return None,
        }
    }
}