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
            current_iterator: 0,
            current_entity: 0,
            array_1: match self.components.get::<ComponentArray<C>>() {
                None => None,
                Some(comp_arr) => Some(comp_arr.get_array()),
            },
            current_index_1: 0,
        };
    }

    pub fn iterate_over_2_component_mut<'a, C1: 'static, C2: 'static>(&'a mut self) -> ComponentIterator2<'a, C1, C2> {
        return ComponentIterator2::<C1, C2> {
            current_iterator: 0,
            current_entity: 0,
            array_1: match self.components.get::<ComponentArray<C1>>() {
                None => None,
                Some(comp_arr) => Some(comp_arr.get_array()),
            },
            current_index_1: 0,
            array_2: match self.components.get::<ComponentArray<C2>>() {
                None => None,
                Some(comp_arr) => Some(comp_arr.get_array()),
            },
            current_index_2: 0,
        };
    }

}


pub struct ComponentIterator1<'a, C> {
    current_iterator: usize,
    current_entity: usize,
    array_1: Option<&'a Vec<IndexedElem<C>>>,
    current_index_1: usize,
}

pub struct ComponentIterator2<'a, C1, C2> {
    current_iterator: usize,
    current_entity: usize,
    array_1: Option<&'a Vec<IndexedElem<C1>>>,
    current_index_1: usize,
    array_2: Option<&'a Vec<IndexedElem<C2>>>,
    current_index_2: usize,
}


impl<'a, C: 'static> Iterator for ComponentIterator1<'a, C> {
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current_iterator {
                0 => {
                    // checking for first component
                    while match self.array_1 {
                        None => return None,
                        Some(array) => {
                            match array.get(self.current_index_1) {
                                None => return None, // out of element on first vec, end of iterator
                                Some(i_elem) => {
                                    // use this to update values
                                    if i_elem.index < self.current_entity {
                                        // true to keep the while loop and increment index
                                        true
                                    }
                                    else {
                                        // if we are bigger than current entity, update entity to match ourselves
                                        if i_elem.index > self.current_entity {
                                            // update entity to align to our component
                                            self.current_entity = i_elem.index;
                                            // reset current iterator because we went to next entity, so need to get again all components
                                            self.current_iterator = 0;
                                            // note that the while loop will end, so the loop will come back to this point
                                            // except it will then go to the else and increment the current iterator
                                            // this is a design choice so this code is similar in every arm of the match on self.current_iterator
                                        }
                                        else {
                                            // check next iterator, we are at the component of current entity
                                            self.current_iterator += 1;
                                        }
                                        false // go to next iterator, so end while loop
                                    }
                                },
                            }
                        }
                    } {
                        // advance current index of array 1 to match with current entity
                        self.current_index_1 += 1;
                    }
                }
                _ => {
                    // here, all arrays index have matched the entity, so let's return the components !
                    let result = Some((
                        match self.array_1 {
                            None => return None, // shouldn't happen, but safety
                            Some(array) => match array.get(self.current_index_1) {
                                None => return None, // shouldn't happen, but safety
                                Some(i_elem) => &i_elem.elem,
                            }
                        }
                    ));
                    // update to next entity for iterator
                    self.current_entity += 1;
                    // reset iterator counter
                    self.current_iterator = 0;

                    return result;
                }
            }
        }
    }
}

impl<'a, C1: 'static, C2: 'static> Iterator for ComponentIterator2<'a, C1, C2> {
    type Item = (&'a C1, &'a C2);
    fn next(&mut self) -> Option<Self::Item> {
        // need a big loop that returns any found pair of components
        loop {
            match self.current_iterator {
                0 => {
                    // checking for first component
                    while match self.array_1 {
                        None => return None,
                        Some(array) => {
                            match array.get(self.current_index_1) {
                                None => return None, // out of element on first vec, end of iterator
                                Some(i_elem) => {
                                    // use this to update values
                                    if i_elem.index < self.current_entity {
                                        // true to keep the while loop and increment index
                                        true
                                    }
                                    else {
                                        // if we are bigger than current entity, update entity to match ourselves
                                        if i_elem.index > self.current_entity {
                                            // update entity to align to our component
                                            self.current_entity = i_elem.index;
                                            // reset current iterator because we went to next entity, so need to get again all components
                                            self.current_iterator = 0;
                                            // note that the while loop will end, so the loop will come back to this point
                                            // except it will then go to the else and increment the current iterator
                                            // this is a design choice so this code is similar in every arm of the match on self.current_iterator
                                        }
                                        else {
                                            // check next iterator, we are at the component of current entity
                                            self.current_iterator += 1;
                                        }
                                        false // go to next iterator, so end while loop
                                    }
                                },
                            }
                        }
                    } {
                        // advance current index of array 1 to match with current entity
                        self.current_index_1 += 1;
                    }
                }
                1 => {
                    // checking for last component
                    while match self.array_2 {
                        None => return None,
                        Some(array) => {
                            match array.get(self.current_index_2) {
                                None => return None, // out of element on first vec, end of iterator
                                Some(i_elem) => {
                                    // use this to update values
                                    if i_elem.index < self.current_entity {
                                        // true to keep the while loop and increment index
                                        true
                                    }
                                    else {
                                        // if we are bigger than current entity, update entity to match ourselves
                                        if i_elem.index > self.current_entity {
                                            // update entity to align to our component
                                            self.current_entity = i_elem.index;
                                            // reset current iterator because we went to next entity, so need to get again all components
                                            self.current_iterator = 0;
                                        }
                                        else {
                                            // check next iterator, we are at the component of current entity
                                            self.current_iterator += 1;
                                        }
                                        false // go to next iterator, so end while loop
                                    }
                                },
                            }
                        }
                    } {
                        // advance current index of array 1 to match with current entity
                        self.current_index_2 += 1;
                    }
                }
                _ => {
                    // here, all arrays index have matched the entity, so let's return the components !
                    let result = Some((
                        match self.array_1 {
                            None => return None, // shouldn't happen, but safety
                            Some(array) => match array.get(self.current_index_1) {
                                None => return None, // shouldn't happen, but safety
                                Some(i_elem) => &i_elem.elem,
                            }
                        },
                        match self.array_2 {
                            None => return None, // shouldn't happen, but safety
                            Some(array) => match array.get(self.current_index_2) {
                                None => return None, // shouldn't happen, but safety
                                Some(i_elem) => &i_elem.elem,
                            }
                        }
                    ));
                    // update to next entity for iterator
                    self.current_entity += 1;
                    // reset iterator counter
                    self.current_iterator = 0;

                    return result;
                }
            }
        }
    }
}

/*
#[macro_export]
macro_rules! iterate_over_component {
    ($ecs:expr; $($comp:ty),+) => {
        
        // create the result struct that will act as an iterator
        struct ComponentIterator<$($comp),+> {
            current_iterator: usize,
            current_entity: usize,
            $(
                array_$comp: Option<&'a Vec<IndexedElem<$comp>>>,
                current_index_$comp: usize,
            )*
        }
    };
}
*/