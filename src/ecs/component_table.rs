extern crate anymap;
use crate::utils::collections::packed_array::IndexedElem;
use crate::ecs::{
    entity::Entity,
    component_array::ComponentArray,
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

    pub fn iterate_over_1_component_mut<'a, C: 'static>(&'a self) -> ComponentIterator1<'a, C> {
        return ComponentIterator1::<'a, C> {
            array: match self.components.get::<ComponentArray<C>>() {
                None => None,
                Some(comp_arr) => Some(comp_arr.get_array()),
            },
            current_index: 0,
        };
    }

    /*
    pub fn iterate_over_2_component<'a, C1: 'static, C2: 'static>(&'a mut self) -> Option<ComponentIterator_2<'a, C1, C2>> {
        return Some(ComponentIterator_2::<C1, C2> {
            iterator_1: match self.components.get_mut::<PackedArray<C1>>() {
                None => return None,
                Some(packed_array) => packed_array.iter_mut(),
            },
            iterator_2: match self.components.get_mut::<PackedArray<C2>>() {
                None => return None,
                Some(packed_array) => packed_array.iter_mut(),
            },
            current_entity: 0,
        });
    }
    */
}


pub struct ComponentIterator1<'a, C> {
    array: Option<&'a Vec<IndexedElem<C>>>,
    current_index: usize,
}


impl<'a, C: 'static> Iterator for ComponentIterator1<'a, C> {
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        return match self.array {
            None => None,
            Some(array) => match array.get(self.current_index) {
                None => None,
                Some(elem) => {
                    self.current_index += 1;
                    Some(&elem.elem)
                }
            }
        }
    }
}