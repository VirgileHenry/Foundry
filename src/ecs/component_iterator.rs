/// macro used internally in the iterator macros.
#[macro_export]
macro_rules! fn_internal_get_next_elem {
    ($elem_type:ty; $first:path, $($elems:path),*) => {
        fn macro_generated_return_next(elem: $elem_type) -> $elem_type {
            match elem {
                $first => 
                $($elems, $elems =>)*
                $first
            }
        }

        fn macro_generated_reset() -> $elem_type {
            $first
        }
    }
}


/// Creates an iterator over any n-uplets of components. 
/// For example, we can iterate over every entity that have both a position and a velocity like so:
/// ```for (pos, vel) in iterate_over_component!(components; Position, Velocity) { /* */ }```
#[macro_export]
macro_rules! iterate_over_component {
    ($components:expr; $($comp:ident),+) => {
        {
            use foundry::ecs::component_table::ComponentTable;
            use foundry::utils::collections::packed_array::IndexedElem;
            use std::slice::Iter;


            // use an enum to get an id per component !
            #[derive(Copy, Clone)]
            enum MacroGeneratedComponentsEnum {
                $(
                    $comp,
                )+
                EndOfIterator
            }

            struct MacroGeneratedComponentIterator<'a, $($comp),+> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                active_entities: &'a Vec<u8>,
                $(
                    $comp: std::iter::Peekable<Iter<'a, IndexedElem<$comp>>>
                ),+
            }

            use foundry::fn_internal_get_next_elem;
            // generate methods to go to next components enum
            fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
            
            impl<'a, $($comp: 'static),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                type Item = ($(&'a $comp),+);
                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        match self.current_component {
                            $(
                                MacroGeneratedComponentsEnum::$comp => {
                                    while match &self.$comp.peek() {
                                        None => return None,
                                        Some(elem) => {
                                            if elem.index > self.current_entity {
                                                self.current_entity = elem.index;
                                                false
                                            }
                                            else if elem.index == self.current_entity {
                                                if match self.active_entities.get(self.current_entity / 8) {
                                                    Some(pack) => {
                                                        (pack & (1 << (self.current_entity % 8))) == 0
                                                    }
                                                    None => return None, // no more entities to read
                                                } {
                                                    self.current_entity += 1; // current entity is inactive, go to next one
                                                    self.current_component = macro_generated_reset();
                                                }
                                                else {
                                                    self.current_component = macro_generated_return_next(self.current_component);
                                                }
                                                false
                                            }
                                            else {
                                                true
                                            }
                                        }
                                    } {
                                        self.$comp.next();
                                    }
                                }
                            )+
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = ($(
                                    match self.$comp.next() {
                                        Some(elem) => & elem.elem,
                                        None => return None,
                                    }
                                ),+);
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }

            let mut result = MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component: macro_generated_reset(),
                active_entities: ComponentTable::get_active_entities(&$components),
                $(
                    $comp: match ComponentTable::get_component_array_mut::<$comp>(&$components) {
                        Some(comp_arr) => comp_arr.iter().peekable(),
                        None => [].iter().peekable(),
                    }
                ),+
            };

            result

        }
    };
}


/// Creates an iterator over any n-uplets of components. 
/// For example, we can iterate over every entity that have both a position and a velocity like so:
/// ```for (pos, vel) in iterate_over_component!(components; Position, Velocity) { /* */ }```
/// Warning : there is currently a safety issue that allows to make an inner loop also iterating over the same component type.
/// it is possible to do ```for pos1 in iterate_over_component_mut!(comp, Position) { for pos2 in iterate_over_component_mut!(comp, Position) { /* pos1 and pos2 refer to the same data at some point */} } ```
#[macro_export]
macro_rules! iterate_over_component_mut {
    ($components:expr; $($comp:ident),+) => {
        {
            use foundry::ecs::component_table::ComponentTable;
            use foundry::utils::collections::packed_array::IndexedElem;
            use std::slice::IterMut;


            // use an enum to get an id per component !
            #[derive(Copy, Clone)]
            enum MacroGeneratedComponentsEnum {
                $(
                    $comp,
                )+
                EndOfIterator
            }

            struct MacroGeneratedComponentIterator<'a, $($comp),+> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                active_entities: &'a Vec<u8>,
                $(
                    $comp: std::iter::Peekable<IterMut<'a, IndexedElem<$comp>>>
                ),+
            }

            use foundry::fn_internal_get_next_elem;

            // generate methods to go to next components enum
            fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
            
            impl<'a, $($comp),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                type Item = ($(&'a mut $comp),+);
                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        match self.current_component {
                            $(
                                MacroGeneratedComponentsEnum::$comp => {
                                    while match &self.$comp.peek() {
                                        None => return None,
                                        Some(elem) => {
                                            if elem.index > self.current_entity {
                                                self.current_entity = elem.index;

                                                false
                                            }
                                            else if elem.index == self.current_entity {
                                                if match self.active_entities.get(self.current_entity / 8) {
                                                    Some(pack) => {
                                                        (pack & (1 << (self.current_entity % 8))) == 0
                                                    }
                                                    None => return None, // no more entities to read
                                                } {
                                                    self.current_entity += 1; // current entity is inactive, go to next one
                                                    self.current_component = macro_generated_reset();
                                                }
                                                else {
                                                    self.current_component = macro_generated_return_next(self.current_component);
                                                }
                                                false
                                            }
                                            else {
                                                true
                                            }
                                        }
                                    } {
                                        self.$comp.next();
                                    }
                                }
                            )+
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = ($(
                                    match self.$comp.next() {
                                        Some(elem) => & mut elem.elem,
                                        None => return None,
                                    }
                                ),+);
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }

            let mut result = MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component: macro_generated_reset(),
                active_entities: ComponentTable::get_active_entities(&$components),
                $(
                    $comp: match ComponentTable::get_component_array_mut::<$comp>(&$components) {
                        Some(comp_arr) => comp_arr.iter_mut().peekable(),
                        None => [].iter_mut().peekable(),
                    }
                ),+
            };

            result

        }
    };
}
