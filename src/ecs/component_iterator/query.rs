
#[macro_export]
macro_rules! generate_query {
    ($name:ident, $state_name:ident, $($types:tt),+) => {

        paste::paste! {
            #[derive(Clone, Copy)]
            enum $state_name {
                $(
                    $types,
                )+
                End,
            }
            crate::query_state_funcs!($state_name, $(
                $state_name::$types,
            )+ $state_name::End);
    
            pub struct $name<'a, $($types),+> {
                current_entity: usize,
                mask: u32,
                query_state: $state_name,
                active_entities: &'a FoundryBoolVecInner,
                entity_layers: &'a FoundryEntityMasks,
                $(
                    pub [<$types:snake>]: std::iter::Peekable<std::slice::Iter<'a, FoundryIndexedElemInner<$types>>>,
                )+
            }

            impl<'a, $($types),+> Iterator for $name<'a, $($types),+> {
                #[allow(unused_parens)]
                type Item = (usize, $(&'a $types),+);

                fn next(&mut self) -> Option<Self::Item> {
                    while !self.active_entities.get(self.current_entity)? || (self.entity_layers.get(self.current_entity)? & self.mask) == 0 {
                        self.current_entity += 1;
                    }
                    loop {
                        match self.query_state {
                            $(
                                $state_name::$types => {
                                    while self.[<$types:snake>].peek()?.index() < self.current_entity {
                                        self.[<$types:snake>].next();
                                    }
                                    if self.[<$types:snake>].peek()?.index() > self.current_entity {
                                        self.current_entity = self.[<$types:snake>].peek()?.index();
                                        self.query_state = $state_name::reset_state();
                                    } else { 
                                        self.query_state = self.query_state.next_state();
                                    }
                                }
                            ),+
                            $state_name::End => {
                                let result = (
                                    self.current_entity,
                                    $(
                                        self.[<$types:snake>].next()?.elem()
                                    ),+
                                );
                                self.current_entity += 1;
                                self.query_state = $state_name::reset_state();
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
macro_rules! impl_query {
    ($name:ident, $state_name:ident, $($types:tt),+) => {
        impl crate::ComponentTable {
            paste::paste! {
                pub fn [<$name:snake>]<'a, $($types: 'static),+>(&'a self) -> impl Iterator<Item = (usize, $(&'a $types),+)> {
                    $name {
                        current_entity: 0,
                        mask: u32::MAX,
                        query_state: $state_name::reset_state(),
                        active_entities: self.get_active_entities(),
                        entity_layers: self.get_entity_layers(),
                        $(
                            [<$types:snake>]: match self.get_component_array::<$types>() {
                                Some(arr) => arr.iter().peekable(),
                                None => [].iter().peekable(),
                            }
                        ),+
                    }
                }
            }
        }
    };
}