


#[macro_export]
macro_rules! iterate_over_component {
    ($ecs:expr; $($comp:ident),+) => {
        {
            // get the comp map once to avoid multi borrow issues (we have unsafe cell for vectors)
            let mut comp_map = ECS::get_unsafe_component_map($ecs);

            internal_iterate_over_component!(comp_map; $($comp),+)
        }
    }
}

#[macro_export]
macro_rules! iterate_over_component_from_sys {
    ($components:expr; $($comp:ident),+) => {
        {
            use crate::ecs::component_table::ComponentTable;
            // get the comp map once to avoid multi borrow issues (we have unsafe cell for vectors)
            let mut comp_map = ComponentTable::get_component_map($components);

            internal_iterate_over_component!(comp_map; $($comp),+)
        }
    }
}


#[macro_export]
macro_rules! iterate_over_component_mut {
    ($ecs:expr; $($comp:ident),+) => {
        {
            // get the comp map once to avoid multi borrow issues (we have unsafe cell for vectors)
            let mut comp_map = ECS::get_unsafe_component_map($ecs);

            internal_iterate_over_component_mut!(comp_map; $($comp),+)
        }
    }
}

#[macro_export]
macro_rules! iterate_over_component_from_sys_mut {
    ($components:expr; $($comp:ident),+) => {
        {
            use crate::ecs::component_table::ComponentTable;
            // get the comp map once to avoid multi borrow issues (we have unsafe cell for vectors)
            let mut comp_map = ComponentTable::get_component_map($components);

            internal_iterate_over_component_mut!(comp_map; $($comp),+)
        }
    }
}


#[macro_export]
macro_rules! internal_iterate_over_component {
    ($comp_map:expr; $($comp:ident),+) => {
        {
            // use statments to import everything that is needed
            use crate::utils::collections::packed_array::IndexedElem;
            use crate::ecs::component_array::ComponentArray;

            // use an enum to get an id per component !
            #[derive(Copy, Clone)]
            enum MacroGeneratedComponentsEnum {
                $(
                    $comp,
                )+
                EndOfIterator
            }

            // generate methods to go to next components enum
            fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);

            // struct to pack both a vec and a index
            struct MacroGeneratedIterableVec<'a, T> {
                vec: Option<&'a Vec<IndexedElem<T>>>,
                index: usize,
            }
            // create the result struct that will act as an iterator
            struct MacroGeneratedComponentIterator<'a, $($comp),+> {
                current_iterator: MacroGeneratedComponentsEnum,
                current_entity: usize,
                $(
                    $comp: MacroGeneratedIterableVec<'a, $comp>,
                )+
            }

            // implement the iterator 
            impl<'a, $($comp),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                type Item = ($(&'a $comp),+);
                fn next(&mut self) -> Option<Self::Item> {
                    // a bit tricky but allows static enums as values :
                    // create usize vars of name comp that store their enums values
                    $(
                        let $comp: usize = MacroGeneratedComponentsEnum::$comp as usize;
                    )+
                    loop {
                        match self.current_iterator {
                            $(
                                MacroGeneratedComponentsEnum::$comp => {
                                    // checking for first component
                                    while match self.$comp.vec {
                                        None => return None,
                                        Some(array) => {
                                            match array.get(self.$comp.index) {
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
                                                            self.current_iterator = macro_generated_reset();
                                                            // note that the while loop will end, so the loop will come back to this point
                                                            // except it will then go to the else and increment the current iterator
                                                            // this is a design choice so this code is similar in every arm of the match on self.current_iterator
                                                        }
                                                        else {
                                                            // check next iterator, we are at the component of current entity
                                                            self.current_iterator = macro_generated_return_next(self.current_iterator);
                                                        }
                                                        false // go to next iterator, so end while loop
                                                    }
                                                },
                                            }
                                        }
                                    } {
                                        // advance current index of array 1 to match with current entity
                                        self.$comp.index += 1;
                                    }
                                }
                            )+
                            _ =>{
                                                    // here, all arrays index have matched the entity, so let's return the components !
                                let result = Some((
                                    $(
                                        match self.$comp.vec {
                                            None => return None, // shouldn't happen, but safety
                                            Some(array) => match array.get(self.$comp.index) {
                                                None => return None, // shouldn't happen, but safety
                                                Some(i_elem) => &i_elem.elem,
                                            }
                                        }
                                    ),+
                                ));
                                // update to next entity for iterator
                                self.current_entity += 1;
                                // reset iterator counter
                                self.current_iterator = macro_generated_reset();
                            
                                return result;
                            }
                        }
                    }
                }
            }

            MacroGeneratedComponentIterator::<$($comp),+> {
                current_iterator: macro_generated_reset(),
                current_entity: 0,
                $(
                    $comp: MacroGeneratedIterableVec {
                        vec: match $comp_map.get::<ComponentArray<$comp>>() {
                            None => None,
                            Some(comp_arr) => Some(comp_arr.get_array()),
                        },
                        index: 0,
                    },
                )+
            }
        }
    };
}



#[macro_export]
macro_rules! internal_iterate_over_component_mut {
    ($comp_map:expr; $($comp:ident),+) => {
        {
            // use statments to import everything that is needed
            use crate::utils::collections::packed_array::IndexedElem;
            use crate::ecs::component_array::ComponentArray;

            // use an enum to get an id per component !
            #[derive(Copy, Clone)]
            enum MacroGeneratedComponentsEnum {
                $(
                    $comp,
                )+
                EndOfIterator
            }

            // generate methods to go to next components enum
            fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);

            // struct to pack both a vec and a index
            struct MacroGeneratedIterableVec<'a, T> {
                vec: Option<&'a mut Vec<IndexedElem<T>>>,
                index: usize,
            }
            // create the result struct that will act as an iterator
            struct MacroGeneratedComponentIterator<'a, $($comp),+> {
                current_iterator: MacroGeneratedComponentsEnum,
                current_entity: usize,
                $(
                    $comp: MacroGeneratedIterableVec<'a, $comp>,
                )+
            }

            // implement the iterator 
            impl<'a, $($comp),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                type Item = ($(&'a mut $comp),+);
                fn next(&mut self) -> Option<Self::Item> {
                    // a bit tricky but allows static enums as values :
                    // create usize vars of name comp that store their enums values
                    $(
                        let $comp: usize = MacroGeneratedComponentsEnum::$comp as usize;
                    )+
                    loop {
                        match self.current_iterator {
                            $(
                                MacroGeneratedComponentsEnum::$comp => {
                                    // checking for first component
                                    while match &self.$comp.vec {
                                        None => return None,
                                        Some(array) => {
                                            match array.get(self.$comp.index) {
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
                                                            self.current_iterator = macro_generated_reset();
                                                            // note that the while loop will end, so the loop will come back to this point
                                                            // except it will then go to the else and increment the current iterator
                                                            // this is a design choice so this code is similar in every arm of the match on self.current_iterator
                                                        }
                                                        else {
                                                            // check next iterator, we are at the component of current entity
                                                            self.current_iterator = macro_generated_return_next(self.current_iterator);
                                                        }
                                                        false // go to next iterator, so end while loop
                                                    }
                                                },
                                            }
                                        }
                                    } {
                                        // advance current index of array 1 to match with current entity
                                        self.$comp.index += 1;
                                    }
                                }
                            )+
                            _ =>{
                                // here, all arrays index have matched the entity, so let's return the components !
                                let result = Some((
                                    $(
                                        match &self.$comp.vec {
                                            None => return None, // shouldn't happen, but safety
                                            Some(array) => match array.get_mut(self.$comp.index) {
                                                None => return None, // shouldn't happen, but safety
                                                Some(i_elem) => &mut i_elem.elem,
                                            }
                                        }
                                    ),+
                                ));
                                // update to next entity for iterator
                                self.current_entity += 1;
                                // reset iterator counter
                                self.current_iterator = macro_generated_reset();
                            
                                return result;
                            }
                        }
                    }
                }
            }

            MacroGeneratedComponentIterator::<$($comp),+> {
                current_iterator: macro_generated_reset(),
                current_entity: 0,
                $(
                    $comp: MacroGeneratedIterableVec {
                        vec: match $comp_map.get::<ComponentArray<$comp>>() {
                            None => None,
                            Some(comp_arr) => Some(comp_arr.get_array_mut()),
                        },
                        index: 0,
                    },
                )+
            }
        }
    };
}
