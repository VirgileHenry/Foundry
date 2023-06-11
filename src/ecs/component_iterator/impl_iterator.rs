/// WARNING INNER MACRO : do not call by yourself.
/// Implements the iterator trait for our result component iterator.
/// With the mut keywords, we have to do this in the same way : 
/// put the whole empty implementation as a param, and it will complete itslef with recursive calls.
#[macro_export]
macro_rules! impl_iterator_inner {
    // we are adding a non terminal non mutable component.
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            #[allow(unused_parens)] // case of a single comp given.
            type Item = (foundry::Entity, ($($item_out:tt)*));
            fn next(&mut $self:ident) -> Option<Self::Item> {
                while !self.active_entities.get(self.current_entity)? || (self.entity_layers.get(self.current_entity)? & self.entity_mask) == 0 {
                    self.current_entity += 1;
                }
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (self.current_entity, ($($result_out:tt)*));
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
                    #[allow(unused_parens)] // case of a single comp given.
                    type Item = (foundry::Entity, ($($item_out)* &'a $comp,));
                    fn next(&mut $self) -> Option<Self::Item> {
                        // advance current entity while it is inactive or does not match the mask
                        while !$self.active_entities.get($self.current_entity)? || ($self.entity_layers.get($self.current_entity)? & $self.entity_mask) == 0 {
                            $self.current_entity += 1;
                        }
                        // at this point, we are sure to have an active entity with a matching mask
                        loop {
                            match $self.current_component {
                                (
                                    $($match_out)*
                                    MacroGeneratedComponentsEnum::[<$comp:camel>] => {
                                        // advance until we go at or pass entity
                                        while $self.[<$comp:snake>].peek()?.index() < $self.current_entity {
                                            $self.[<$comp:snake>].next();
                                        }
                                        // check what we do with current entity :
                                        if $self.[<$comp:snake>].peek()?.index() > $self.current_entity {
                                            // update the current entity to next entity to have this comp
                                            $self.current_entity = $self.[<$comp:snake>].peek()?.index();
                                            $self.current_component = macro_generated_reset();
                                        } else {
                                            // we are valid on this component, 
                                            $self.current_component = macro_generated_return_next($self.current_component)
                                        }
                                    }
                                )
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        $self.current_entity,
                                        ($($result_out)*
                                        $self.[<$comp:snake>].next()?.elem(),)
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
            #[allow(unused_parens)] // case of a single comp given.
            type Item = (foundry::Entity, ($($item_out:tt)*));
            fn next(&mut $self:ident) -> Option<Self::Item> {
                while !self.active_entities.get(self.current_entity)? || (self.entity_layers.get(self.current_entity)? & self.entity_mask) == 0 {
                    self.current_entity += 1;
                }
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (self.current_entity, ($($result_out:tt)*));
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
                #[allow(unused_parens)] // case of a single comp given.
                type Item = (foundry::Entity, ($($item_out)* &'a $comp));
                fn next(&mut $self) -> Option<Self::Item> {
                    // advance current entity while it is inactive or does not match the mask
                    while !$self.active_entities.get($self.current_entity)? || ($self.entity_layers.get($self.current_entity)? & $self.entity_mask) == 0 {
                        $self.current_entity += 1;
                    }
                    // at this point, ..we are sure to have an active entity with a matching mask
                    loop {
                        match $self.current_component {
                            $($match_out)*
                            MacroGeneratedComponentsEnum::[<$comp:camel>] => {
                                // advance until we go at or pass entity
                                while $self.[<$comp:snake>].peek()?.index() < $self.current_entity {
                                    $self.[<$comp:snake>].next();
                                }
                                // check what we do with current entity :
                                if $self.[<$comp:snake>].peek()?.index() > $self.current_entity {
                                    // update the current entity to next entity to have this comp
                                    $self.current_entity = $self.[<$comp:snake>].peek()?.index();
                                    $self.current_component = macro_generated_reset();
                                } else {
                                    // we are valid on this component, 
                                    $self.current_component = macro_generated_return_next($self.current_component)
                                }
                            }
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (
                                    $self.current_entity,
                                    ($($result_out)*
                                    $self.[<$comp:snake>].next()?.elem())
                                );
                                $self.current_entity += 1;
                                $self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }
        }
    };
    // we are adding a non terminal mutable component.
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            #[allow(unused_parens)] // case of a single comp given.
            type Item = (foundry::Entity, ($($item_out:tt)*));
            fn next(&mut $self:ident) -> Option<Self::Item> {
                while !self.active_entities.get(self.current_entity)? || (self.entity_layers.get(self.current_entity)? & self.entity_mask) == 0 {
                    self.current_entity += 1;
                }
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (self.current_entity, ($($result_out:tt)*));
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
                    #[allow(unused_parens)] // case of a single comp given.
                    type Item = (foundry::Entity, ($($item_out)* &'a mut $comp,));
                    fn next(&mut $self) -> Option<Self::Item> {
                        // advance current entity while it is inactive or does not match the mask
                        while !$self.active_entities.get($self.current_entity)? || ($self.entity_layers.get($self.current_entity)? & $self.entity_mask) == 0 {
                            $self.current_entity += 1;
                        }
                        // at this point, we are sure to have an active entity with a matching mask
                        loop {
                            match $self.current_component {
                                (
                                    $($match_out)*
                                    MacroGeneratedComponentsEnum::[<$comp:camel>] => {
                                        // advance until we go at or pass entity
                                        while $self.[<$comp:snake>].peek()?.index() < $self.current_entity {
                                            $self.[<$comp:snake>].next();
                                        }
                                        // check what we do with current entity :
                                        if $self.[<$comp:snake>].peek()?.index() > $self.current_entity {
                                            // update the current entity to next entity to have this comp
                                            $self.current_entity = $self.[<$comp:snake>].peek()?.index();
                                            $self.current_component = macro_generated_reset();
                                        } else {
                                            // we are valid on this component, 
                                            $self.current_component = macro_generated_return_next($self.current_component)
                                        }
                                    }
                                )
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        $self.current_entity,
                                        ($($result_out)*
                                        $self.[<$comp:snake>].next()?.elem_mut(),)
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
    // we are adding a terminal mutable component.
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            #[allow(unused_parens)] // case of a single comp given.
            type Item = (foundry::Entity, ($($item_out:tt)*));
            fn next(&mut $self:ident) -> Option<Self::Item> {
                while !self.active_entities.get(self.current_entity)? || (self.entity_layers.get(self.current_entity)? & self.entity_mask) == 0 {
                    self.current_entity += 1;
                }
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = (self.current_entity, ($($result_out:tt)*));
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
                #[allow(unused_parens)] // case of a single comp given.
                type Item = (foundry::Entity, ($($item_out)* &'a mut $comp));
                fn next(&mut $self) -> Option<Self::Item> {
                    // advance current entity while it is inactive or does not match the mask
                    while !$self.active_entities.get($self.current_entity)? || ($self.entity_layers.get($self.current_entity)? & $self.entity_mask) == 0 {
                        $self.current_entity += 1;
                    }
                    // at this point, we are sure to have an active entity with a matching mask
                    loop {
                        match $self.current_component {
                            $($match_out)*
                            MacroGeneratedComponentsEnum::[<$comp:camel>] => {
                                // advance until we go at or pass entity
                                while $self.[<$comp:snake>].peek()?.index() < $self.current_entity {
                                    $self.[<$comp:snake>].next();
                                }
                                // check what we do with current entity :
                                if $self.[<$comp:snake>].peek()?.index() > $self.current_entity {
                                    // update the current entity to next entity to have this comp
                                    $self.current_entity = $self.[<$comp:snake>].peek()?.index();
                                    $self.current_component = macro_generated_reset();
                                } else {
                                    // we are valid on this component, 
                                    $self.current_component = macro_generated_return_next($self.current_component)
                                }
                            }
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (
                                    $self.current_entity,
                                    ($($result_out)*
                                    $self.[<$comp:snake>].next()?.elem_mut())
                                );
                                $self.current_entity += 1;
                                $self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }
        }
    };
}

/// WARNING INNER MACRO : do not call by yourself.
/// This calls the inner impl iterator macro, passing it the correct start arguments.
#[macro_export]
macro_rules! impl_iterator {
    ($($comps:tt)+) => {
        impl_iterator_inner!(
            impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
                #[allow(unused_parens)] // case of a single comp given.
                type Item = (foundry::Entity, ());
                fn next(&mut self) -> Option<Self::Item> {
                    while !self.active_entities.get(self.current_entity)? || (self.entity_layers.get(self.current_entity)? & self.entity_mask) == 0 {
                        self.current_entity += 1;
                    }
                    loop {
                        match self.current_component {
                            ()
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (self.current_entity, ());
                                self.current_entity += 1;
                                self.current_component = macro_generated_reset();
                                return Some(result);
                            }
                        }
                    }
                }
            }, $($comps)+
        );
    }
}