
/// macro for intern manipulation of the iterator macros.
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
            foundry::paste::paste! {
                // an enum to get an id per component !
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
                    active_entities: &'a foundry::BoolVec,
                    $(
                        [<$comp:snake>]: std::iter::Peekable<std::slice::Iter<'a, foundry::IndexedElem<$comp>>>
                    ),+
                }

                // generate methods to go to next components enum
                foundry::fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
                
                impl<'a, $($comp: 'static),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                    type Item = ($(&'a $comp),+);
                    fn next(&mut self) -> Option<Self::Item> {
                        loop {
                            match self.current_component {
                                $(
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while match &self.[<$comp:snake>].peek() {
                                            None => return None,
                                            Some(elem) => {
                                                if elem.index > self.current_entity {
                                                    self.current_entity = elem.index; // update the current entity
                                                    self.current_component = macro_generated_reset();
                                                    false
                                                }
                                                else if elem.index == self.current_entity {
                                                    if match self.active_entities.get(self.current_entity) {
                                                        Some(is_active) => {
                                                            !is_active // become the if condition
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
                                            self.[<$comp:snake>].next();
                                        }
                                    }
                                )+
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = ($(
                                        match self.[<$comp:snake>].next() {
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
                    active_entities: foundry::ComponentTable::get_active_entities(&$components),
                    $(
                        [<$comp:snake>]: match foundry::ComponentTable::get_component_array_mut::<$comp>(&$components) {
                            Some(comp_arr) => comp_arr.iter().peekable(),
                            None => [].iter().peekable(),
                        }
                    ),+
                };

                result
            }


        }
    };
    ($components:expr; EntityRef; $($comp:ident),+) => {
        {
            foundry::paste::paste!{

                // an enum to get an id per component !
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
                    active_entities: &'a foundry::BoolVec,
                    $(
                        [<$comp:snake>]: std::iter::Peekable<std::slice::Iter<'a, foundry::IndexedElem<$comp>>>
                    ),+
                }

                // generate methods to go to next components enum
                foundry::fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
                
                impl<'a, $($comp: 'static),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                    type Item = (foundry::EntityRef, $(&'a $comp),+);
                    fn next(&mut self) -> Option<Self::Item> {
                        loop {
                            match self.current_component {
                                $(
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while match &self.[<$comp:snake>].peek() {
                                            None => return None,
                                            Some(elem) => {
                                                if elem.index > self.current_entity {
                                                    self.current_entity = elem.index; // update the current entity
                                                    self.current_component = macro_generated_reset();
                                                    false
                                                }
                                                else if elem.index == self.current_entity {
                                                    if match self.active_entities.get(self.current_entity) {
                                                        Some(is_active) => {
                                                            !is_active // become the if condition
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
                                            self.[<$comp:snake>].next();
                                        }
                                    }
                                )+
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        foundry::EntityRef{id: self.current_entity},
                                        $(
                                        match self.[<$comp:snake>].next() {
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
                    active_entities: foundry::ComponentTable::get_active_entities(&$components),
                    $(
                        [<$comp:snake>]: match foundry::ComponentTable::get_component_array_mut::<$comp>(&$components) {
                            Some(comp_arr) => comp_arr.iter().peekable(),
                            None => [].iter().peekable(),
                        }
                    ),+
                };

                result
            }
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
            foundry::paste::paste!{    

                // an enum to get an id per component !
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
                    active_entities: &'a foundry::BoolVec,
                    $(
                        [<$comp:snake>]: std::iter::Peekable<std::slice::IterMut<'a, foundry::IndexedElem<$comp>>>
                    ),+
                }

                // generate methods to go to next components enum
                foundry::fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
                
                impl<'a, $($comp),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                    type Item = ($(&'a mut $comp),+);
                    fn next(&mut self) -> Option<Self::Item> {
                        loop {
                            match self.current_component {
                                $(
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while match &self.[<$comp:snake>].peek() {
                                            None => return None,
                                            Some(elem) => {
                                                if elem.index > self.current_entity {
                                                    self.current_entity = elem.index; // update the current entity
                                                    self.current_component = macro_generated_reset();
                                                    false
                                                }
                                                else if elem.index == self.current_entity {
                                                    if match self.active_entities.get(self.current_entity) {
                                                        Some(is_active) => {
                                                            !is_active // become the if condition
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
                                            self.[<$comp:snake>].next();
                                        }
                                    }
                                )+
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = ($(
                                        match self.[<$comp:snake>].next() {
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
                    active_entities: foundry::ComponentTable::get_active_entities(&$components),
                    $(
                        [<$comp:snake>]: match foundry::ComponentTable::get_component_array_mut::<$comp>(&$components) {
                            Some(comp_arr) => comp_arr.iter_mut().peekable(),
                            None => [].iter_mut().peekable(),
                        }
                    ),+
                };

                result
            }

        }
    };
    ($components:expr; EntityRef; $($comp:ident),+) => {
        {
            foundry::paste::paste!{
                // an enum to get an id per component !
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
                    active_entities: &'a foundry::BoolVec,
                    $(
                        [<$comp:snake>]: std::iter::Peekable<std::slice::IterMut<'a, foundry::IndexedElem<$comp>>>
                    ),+
                }

                // generate methods to go to next components enum
                foundry::fn_internal_get_next_elem!(MacroGeneratedComponentsEnum; $(MacroGeneratedComponentsEnum::$comp, )+ MacroGeneratedComponentsEnum::EndOfIterator);
                
                impl<'a, $($comp),+> Iterator for MacroGeneratedComponentIterator<'a, $($comp),+> {
                    type Item = (foundry::EntityRef, $(&'a mut $comp),+);
                    fn next(&mut self) -> Option<Self::Item> {
                        loop {
                            match self.current_component {
                                $(
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while match &self.[<$comp:snake>].peek() {
                                            None => return None,
                                            Some(elem) => {
                                                if elem.index > self.current_entity {
                                                    self.current_entity = elem.index; // update the current entity
                                                    self.current_component = macro_generated_reset();
                                                    false
                                                }
                                                else if elem.index == self.current_entity {
                                                    if match self.active_entities.get(self.current_entity) {
                                                        Some(is_active) => {
                                                            !is_active // become the if condition
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
                                            self.[<$comp:snake>].next();
                                        }
                                    }
                                )+
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        foundry::EntityRef{id: self.current_entity},
                                        $(
                                        match self.[<$comp:snake>].next() {
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
                    active_entities: foundry::ComponentTable::get_active_entities(&$components),
                    $(
                        [<$comp:snake>]: match foundry::ComponentTable::get_component_array_mut::<$comp>(&$components) {
                            Some(comp_arr) => comp_arr.iter_mut().peekable(),
                            None => [].iter_mut().peekable(),
                        }
                    ),+
                };

                result
            }
        }
    };
}
