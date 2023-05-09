/// Implements the iterator trait for our result component iterator.
/// With the mut keywords, we have to do this in the same way : 
/// put the whole empty implementation as a param, and it will complete itslef with recursive calls.
#[macro_export]
macro_rules! impl_iterator_inner {
    // we are adding a non terminal non mutable component.
    (
        impl<'a> Iterator for MacroGeneratedComponentIterator<'a> {
            type Item = ($($item_out:tt)*);
            fn next(&mut $self:ident) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = ($($result_out:tt)*);
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
                    fn next(&mut $self) -> Option<Self::Item> {
                        loop {
                            match $self.current_component {
                                (
                                    $($match_out)*
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while {
                                            let elem = &$self.[<$comp:snake>].peek()?;
                                            if elem.index() > $self.current_entity {
                                                $self.current_entity = elem.index(); // update the current entity
                                                $self.current_component = macro_generated_reset();
                                                false
                                            } else if elem.index() == $self.current_entity {
                                                match $self.active_entities.get($self.current_entity)? {
                                                    true => {
                                                        $self.current_entity += 1; // current entity is inactive, go to next one
                                                        $self.current_component = macro_generated_reset();
                                                    }
                                                    false => $self.current_component = macro_generated_return_next($self.current_component),
                                                }
                                                false // stop iteration
                                            } else { true /* keep iterating */ }
                                        } {
                                            $self.[<$comp:snake>].next();
                                        }
                                    }
                                )
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        $($result_out)*
                                        $self.[<$comp:snake>].next()?.elem(),
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
            fn next(&mut $self:ident) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = ($($result_out:tt)*);
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
                fn next(&mut $self) -> Option<Self::Item> {
                    loop {
                        match $self.current_component {
                            $($match_out)*
                            MacroGeneratedComponentsEnum::$comp => {
                                while {
                                    let elem = &$self.[<$comp:snake>].peek()?;
                                    if elem.index() > $self.current_entity {
                                        $self.current_entity = elem.index(); // update the current entity
                                        $self.current_component = macro_generated_reset();
                                        false
                                    } else if elem.index() == $self.current_entity {
                                        match $self.active_entities.get($self.current_entity)? {
                                            true => {
                                                $self.current_entity += 1; // current entity is inactive, go to next one
                                                $self.current_component = macro_generated_reset();
                                            }
                                            false => $self.current_component = macro_generated_return_next($self.current_component),
                                        }
                                        false // stop iteration
                                    } else { true /* keep iterating */ }
                                } {
                                    $self.[<$comp:snake>].next();
                                }
                            }
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (
                                    $($result_out)*
                                    $self.[<$comp:snake>].next()?.elem()
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
            type Item = ($($item_out:tt)*);
            fn next(&mut $self:ident) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = ($($result_out:tt)*);
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
                    fn next(&mut $self) -> Option<Self::Item> {
                        loop {
                            match $self.current_component {
                                (
                                    $($match_out)*
                                    MacroGeneratedComponentsEnum::$comp => {
                                        while {
                                            let elem = &$self.[<$comp:snake>].peek()?;
                                            if elem.index() > $self.current_entity {
                                                $self.current_entity = elem.index(); // update the current entity
                                                $self.current_component = macro_generated_reset();
                                                false
                                            } else if elem.index() == $self.current_entity {
                                                match $self.active_entities.get($self.current_entity)? {
                                                    true => {
                                                        $self.current_entity += 1; // current entity is inactive, go to next one
                                                        $self.current_component = macro_generated_reset();
                                                    }
                                                    false => $self.current_component = macro_generated_return_next($self.current_component),
                                                }
                                                false // stop iteration
                                            } else { true /* keep iterating */ }
                                        } {
                                            $self.[<$comp:snake>].next();
                                        }
                                    }
                                )
                                MacroGeneratedComponentsEnum::EndOfIterator => {
                                    let result = (
                                        $($result_out)*
                                        $self.[<$comp:snake>].next()?.elem_mut(),
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
            type Item = ($($item_out:tt)*);
            fn next(&mut $self:ident) -> Option<Self::Item> {
                loop {
                    match self.current_component {
                        ($($match_out:tt)*)
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            let result = ($($result_out:tt)*);
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
                fn next(&mut $self) -> Option<Self::Item> {
                    loop {
                        match $self.current_component {
                            $($match_out)*
                            MacroGeneratedComponentsEnum::$comp => {
                                while {
                                    let elem = &$self.[<$comp:snake>].peek()?;
                                    if elem.index() > $self.current_entity {
                                        $self.current_entity = elem.index(); // update the current entity
                                        $self.current_component = macro_generated_reset();
                                        false
                                    } else if elem.index() == $self.current_entity {
                                        match $self.active_entities.get($self.current_entity)? {
                                            true => {
                                                $self.current_entity += 1; // current entity is inactive, go to next one
                                                $self.current_component = macro_generated_reset();
                                            }
                                            false => $self.current_component = macro_generated_return_next($self.current_component),
                                        }
                                        false // stop iteration
                                    } else { true /* keep iterating */ }
                                } {
                                    $self.[<$comp:snake>].next();
                                }
                            }
                            MacroGeneratedComponentsEnum::EndOfIterator => {
                                let result = (
                                    $($result_out)*
                                    $self.[<$comp:snake>].next()?.elem_mut()
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
                                let result = ();
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