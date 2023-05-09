
/// macro for intern manipulation of the iterator macros.
#[macro_export]
macro_rules! enum_helper_funcs {
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

/// Inner implementation of the macro to generate the enum.
/// Filters out the 'mut' keywords with recursive calls.
#[macro_export]
macro_rules! generate_enum_inner {
    (($($out:tt)*); mut $comp:ident, $($rest:tt)+) => {
        generate_enum_inner!(($($out)* $comp,); $($rest)+)
    };
    (($($out:tt)*); $comp:ident, $($rest:tt)+) => {
        generate_enum_inner!(($($out)* $comp,); $($rest)+)
    };
    (($($out:tt)*); mut $comp:ident) => {
        generate_enum_inner!(@end ($($out)* $comp))
    };
    (($($out:tt)*); $comp:ident) => {
        generate_enum_inner!(@end $($out)* $comp)
    };
    (@end $($comp:ident),*) => {
        #[derive(Copy, Clone)]
        enum MacroGeneratedComponentsEnum {
            $(
                $comp,
            )+
            EndOfIterator
        }
        // also, let's create the enum helper funcs
        foundry::enum_helper_funcs!(
            MacroGeneratedComponentsEnum;
            $(MacroGeneratedComponentsEnum::$comp,)+
            MacroGeneratedComponentsEnum::EndOfIterator
        );
    };
}

/// Generate the enum iterator that allows to iterate over different component types.
/// We need a separate macro to filter out the 'mut' we get in the input.
#[macro_export]
macro_rules! generate_enum {
    ($($comps:tt)*) => {
        generate_enum_inner!((); $($comps)*)
    };
}

/// This is a bit clanky, but needed. As we have to generate the comp types at a specific spot in the struct,
/// we pass the whole struct as a param in the recursive macro call. Each call will process a bit of the rest,
/// and when we reach the end, return the processed macro this way.
#[macro_export]
macro_rules! generate_result_struct_impl {
    ( struct MacroGeneratedComponentIterator<'a> {
        current_entity: usize,
        current_component: MacroGeneratedComponentsEnum,
        active_entities: &'a foundry::FoundryBoolVecInner,
        ($($fields_out:tt)*)
    }, $comp:ident, $($rest:tt)+) => {
        // recursive call, treating an elem of the rest at a time
        paste::paste! { generate_result_struct_impl!(
            struct MacroGeneratedComponentIterator<'a> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                active_entities: &'a foundry::FoundryBoolVecInner,
                ($($fields_out)* [<$comp:snake>]: std::iter::Peekable<std::slice::Iter<'a, foundry::FoundryIndexedElemInner<$comp>>>,)
            }, $($rest)+
        );
    } };
    ( struct MacroGeneratedComponentIterator<'a> {
        current_entity: usize,
        current_component: MacroGeneratedComponentsEnum,
        active_entities: &'a foundry::FoundryBoolVecInner,
        ($($fields_out:tt)*)
    }, mut $comp:ident, $($rest:tt)+ ) => {
        // recursive call, treating an elem of the rest at a time
        paste::paste! { generate_result_struct_impl!(
            struct MacroGeneratedComponentIterator<'a> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                active_entities: &'a foundry::FoundryBoolVecInner,
                ($($fields_out)* [<$comp:snake>]: std::iter::Peekable<std::slice::IterMut<'a, foundry::FoundryIndexedElemInner<$comp>>>,)
            }, $($rest)+
        );
    } };
    ( struct MacroGeneratedComponentIterator<'a> {
        current_entity: usize,
        current_component: MacroGeneratedComponentsEnum,
        active_entities: &'a foundry::FoundryBoolVecInner,
        ($($fields_out:tt)*)
    }, $comp:ident) => {
        // recursive call, treating an elem of the rest at a time
        paste::paste! {
            struct MacroGeneratedComponentIterator<'a> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                active_entities: &'a foundry::FoundryBoolVecInner,
                $($fields_out)* [<$comp:snake>]: std::iter::Peekable<std::slice::Iter<'a, foundry::FoundryIndexedElemInner<$comp>>>,
            }
        }
    };
    ( struct MacroGeneratedComponentIterator<'a> {
        current_entity: usize,
        current_component: MacroGeneratedComponentsEnum,
        active_entities: &'a foundry::FoundryBoolVecInner,
        ($($fields_out:tt)*)
    }, mut $comp:ident) => {
        // recursive call, treating an elem of the rest at a time
        paste::paste! {
            struct MacroGeneratedComponentIterator<'a> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                active_entities: &'a foundry::FoundryBoolVecInner,
                $($fields_out)* [<$comp:snake>]: std::iter::Peekable<std::slice::IterMut<'a, foundry::FoundryIndexedElemInner<$comp>>>,
            }
        }
    };
}

/// Generates the result struct that will impl Iterator.
#[macro_export]
macro_rules! generate_result_struct {
    ($($comps:tt)*) => {
        generate_result_struct_impl!(
            struct MacroGeneratedComponentIterator<'a> {
                current_entity: usize,
                current_component: MacroGeneratedComponentsEnum,
                active_entities: &'a foundry::FoundryBoolVecInner,
                ()
            }, 
            $($comps)*
        );
    };
}

/// Implements the iterator trait for our result component iterator.
/// With the mut keywords, we have to do this in the same way : 
/// put the whole empty implementation as a param, and it will complete itslef with recursive calls.
#[macro_export]
macro_rules! impl_iterator_inner {
    // we are adding a non terminal non mutable component.
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            type Item = ($($item_out:tt)*);
            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (($($result_out:tt)*));
                            self.current_entity += 1;
                            self.current_component = macro_generated_reset();
                            return Some(result);
                        }
                    }
                }
            }
        }, $comp:ident, $($rest:tt)+
    ) => {
        paste::paste! {           
            impl_iterator_inner!(
                impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
                    type Item = ($($item_out)* &'a $comp,);
                    fn next(&mut self) -> Option<Self::Item> {
                        loop {
                            match self.current_component {
                                (
                                    $($match_out)*
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while {
                                            let elem = &self.[<$comp:snake>].peek()?;
                                            if elem.index > self.current_entity {
                                                self.current_entity = elem.index; // update the current entity
                                                self.current_component = macro_generated_reset();
                                                false
                                            } else if elem.index == self.current_entity {
                                                match self.active_entities.get(self.current_entity)? {
                                                    true => {
                                                        self.current_entity += 1; // current entity is inactive, go to next one
                                                        self.current_component = macro_generated_reset();
                                                    }
                                                    false => self.current_component = macro_generated_return_next(self.current_component),
                                                }
                                                false // stop iteration
                                            } else { true /* keep iterating */ }
                                        } {
                                            self.[<$comp:snake>].next();
                                        }
                                    }
                                )
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        (
                                            $($result_out)*
                                            &self.[<$comp:snake>].next()?.elem,
                                        )
                                    );
                                    self.current_entity += 1;
                                    self.current_component = macro_generated_reset();
                                    return Some(result);
                                }
                            }
                        }
                    }
                }, $($rest)+
            );
        }
    };
    // adding a non terminal mutable component. 
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            type Item = (($($item_out:tt)*));
            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (($($result_out:tt)*));
                            self.current_entity += 1;
                            self.current_component = macro_generated_reset();
                            return Some(result);
                        }
                    }
                }
            }
        }, mut $comp:ident, $($rest:tt)+
    ) => {
        paste::paste! {           
            impl_iterator_inner!(
                impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
                    type Item = ($($item_out)* &'a mut $comp,);
                    fn next(&mut self)
                     -> Option<Self::Item> {
                        loop {
                            match self.current_component {
                                (
                                    $($match_out)*
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while {
                                            let elem = &self.[<$comp:snake>].peek()?;
                                            if elem.index > self.current_entity {
                                                self.current_entity = elem.index; // update the current entity
                                                self.current_component = macro_generated_reset();
                                                false
                                            } else if elem.index == self.current_entity {
                                                match self.active_entities.get(self.current_entity)? {
                                                    true => {
                                                        self.current_entity += 1; // current entity is inactive, go to next one
                                                        self.current_component = macro_generated_reset();
                                                    }
                                                    false => self.current_component = macro_generated_return_next(self.current_component),
                                                }
                                                false // stop iteration
                                            } else { true /* keep iterating */ }
                                        } {
                                            self.[<$comp:snake>].next();
                                        }
                                    },
                                )
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        (
                                            $($result_out)*
                                            &self.[<$comp:snake>].next()?.elem,
                                        )
                                    );
                                    self.current_entity += 1;
                                    self.current_component = macro_generated_reset();
                                    return Some(result);
                                }
                            }
                        }
                    }
                }, $($rest)+
            );
        }
    };
    // we are adding a terminal non mutable component.
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            type Item = ($($item_out:tt)*);
            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (($($result_out:tt)*));
                            self.current_entity += 1;
                            self.current_component = macro_generated_reset();
                            return Some(result);
                        }
                    }
                }
            }
        }, $comp:ident
    ) => {
        paste::paste! {           
            impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
                type Item = ($($item_out)* &'a $comp);
                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        match self.current_component {
                            $($match_out)*
                            MacroGeneratedComponentsEnum::$comp => {
                                while {
                                    let elem = &self.[<$comp:snake>].peek()?;
                                    if elem.index > self.current_entity {
                                        self.current_entity = elem.index; // update the current entity
                                        self.current_component = macro_generated_reset();
                                        false
                                    } else if elem.index == self.current_entity {
                                        match self.active_entities.get(self.current_entity)? {
                                            true => {
                                                self.current_entity += 1; // current entity is inactive, go to next one
                                                self.current_component = macro_generated_reset();
                                            }
                                            false => self.current_component = macro_generated_return_next(self.current_component),
                                        }
                                        false // stop iteration
                                    } else { true /* keep iterating */ }
                                } {
                                    self.[<$comp:snake>].next();
                                }
                            }
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (
                                    (
                                        $($result_out)*
                                        &self.[<$comp:snake>].next()?.elem
                                    )
                                );
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }
        }
    };
    // adding a terminal mutable component. 
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            type Item = ($($item_out:tt)*);
            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (($($result_out:tt)*));
                            self.current_entity += 1;
                            self.current_component = macro_generated_reset();
                            return Some(result);
                        }
                    }
                }
            }
        }, mut $comp:ident
    ) => {
        paste::paste! {           
            impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
                type Item = ($($item_out)* &'a mut $comp);
                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        match self.current_component {
                            $($match_out)*
                            MacroGeneratedComponentsEnum::$comp => {
                                while {
                                    let elem = &self.[<$comp:snake>].peek()?;
                                    if elem.index > self.current_entity {
                                        self.current_entity = elem.index; // update the current entity
                                        self.current_component = macro_generated_reset();
                                        false
                                    } else if elem.index == self.current_entity {
                                        match self.active_entities.get(self.current_entity)? {
                                            true => {
                                                self.current_entity += 1; // current entity is inactive, go to next one
                                                self.current_component = macro_generated_reset();
                                            }
                                            false => self.current_component = macro_generated_return_next(self.current_component),
                                        }
                                        false // stop iteration
                                    } else { true /* keep iterating */ }
                                } {
                                    self.[<$comp:snake>].next();
                                }
                            }
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (
                                    ($($result_out)*)
                                    &self.[<$comp:snake>].next()?.elem
                                );
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_iterator {
    ($($comps:tt)*) => {
        impl_iterator_inner!(
            impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
                type Item = ();
                fn next(&mut self) -> Option<Self::Item> {
                    loop {
                        match self.current_component {
                            ()
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (());
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }, $($comps)*
        );
    }
}

#[macro_export]
macro_rules! create_result_inner {
    (
        MacroGeneratedComponentIterator {
            current_entity: 0,
            current_component,
            active_entities,
            ($($out:tt)*)
        }, $comp:ident, $($rest:tt)*
    ) => {
        MacroGeneratedComponentIterator {
            current_entity: 0,
            current_component: macro_generated_reset(),
            active_entities: foundry::ComponentTable::get_active_entities(&$components),
            (
                $($out:tt)*
                [<$comp:snake>]: match foundry::ComponentTable::get_component_array_mut::<$comp>(&$components) {
                    Some(comp_arr) => comp_arr.iter().peekable(),
                    None => [].iter().peekable(),
                },
            )
        }  
    };
    (
        $components:expr;
        MacroGeneratedComponentIterator {
            current_entity: 0,
            current_component,
            active_entities,
            ($($out:tt)*)
        }, $comp:ident
    ) => {
        paste::paste! {
            MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component: macro_generated_reset(),
                active_entities: foundry::ComponentTable::get_active_entities(&$components),
                $($out:tt)*
                [<$comp:snake>]: match foundry::ComponentTable::get_component_array_mut::<$comp>(&$components) {
                    Some(comp_arr) => comp_arr.iter().peekable(),
                    None => [].iter().peekable(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_result {
    ($components:expr, $($comps:tt)+) => {
        // call the result inner gen
        create_result_inner!(
            $components;
            MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component,
                active_entities,
                ()
            }, $($comps)*
        );
    };
}

#[macro_export]
macro_rules! component_iterator {
    ($components:expr, Entity, $mask:expr; $($comps:tt)+) => {
        {
            // static assert that the types are mutually exclusive
            assert_exclusive_types!($($comps)+);
            // generate the enum used for the iterator
            generate_enum!($($comps)+);
            // generate the result struct with correct mutability
            generate_result_struct!($($comps)+);
            // impl iterator for our result struct
            impl_iterator!($($comps)+);
            // finally, create an instance of the result struct
            create_result!($components, $($comps)+)
        }
    };
}
