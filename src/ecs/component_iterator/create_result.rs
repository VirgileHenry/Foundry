/// WARNING INNER MACRO : do not call by yourself.
/// inner part if the macro that create the result struct.
#[macro_export]
macro_rules! create_result_inner {
    (
        $components:expr;
        $ent_mask:expr;
        MacroGeneratedComponentIterator {
            entity_mask: 0,
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            entity_layers: &entity_layers:expr,
            ($($out:tt)*)
        }, $comp:ident, $($rest:tt)+
    ) => {
        paste::paste! {
            create_result_inner!(
                $components;
                $ent_mask;
                MacroGeneratedComponentIterator {
                    entity_mask: 0,
                    current_entity: 0,
                    current_component: $current_comp,
                    active_entities: $active_entities,
                    entity_layers: $entity_layers,
                    (
                        $($out)*
                        [<$comp:snake>]: match foundry::ComponentTable::get_component_array::<$comp>($components) {
                            Some(comp_arr) => comp_arr.iter().peekable(),
                            None => [].iter().peekable(),
                        },
                    )
                }, $($rest)+
            )
        }
    };
    (
        $components:expr;
        $ent_mask:expr;
        MacroGeneratedComponentIterator {
            entity_mask: 0,
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            entity_layers: $entity_layers:expr,
            ($($out:tt)*)
        }, $comp:ident
    ) => {
        paste::paste! {
            MacroGeneratedComponentIterator {
                entity_mask: $ent_mask,
                current_entity: 0,
                current_component: $current_comp,
                active_entities: $active_entities,
                entity_layers: $entity_layers,
                $($out)*
                [<$comp:snake>]: match foundry::ComponentTable::get_component_array::<$comp>($components) {
                    Some(comp_arr) => comp_arr.iter().peekable(),
                    None => [].iter().peekable(),
                }
            }
        }
    };
    (
        $components:expr;
        $ent_mask:expr;
        MacroGeneratedComponentIterator {
            entity_mask: 0,
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            entity_layers: $entity_layers:expr,
            ($($out:tt)*)
        }, mut $comp:ident, $($rest:tt)+
    ) => {
        paste::paste! {
            create_result_inner!(
                $components;
                $ent_mask;
                MacroGeneratedComponentIterator {
                    entity_mask: 0,
                    current_entity: 0,
                    current_component: $current_comp,
                    active_entities: $active_entities,
                    entity_layers: $entity_layers,
                    (
                        $($out:tt)*
                        [<$comp:snake>]: match unsafe {foundry::ComponentTable::get_component_array_mut::<$comp>($components)} {
                            Some(comp_arr) => comp_arr.iter_mut().peekable(),
                            None => [].iter_mut().peekable(),
                        },
                    )
                }, $($rest)+
            )
        }
    };
    (
        $components:expr;
        $ent_mask:expr;
        MacroGeneratedComponentIterator {
            entity_mask: 0,
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            entity_layers: $entity_layers:expr,
            ($($out:tt)*)
        }, mut $comp:ident
    ) => {
        paste::paste! {
            MacroGeneratedComponentIterator {
                entity_mask: $ent_mask,
                current_entity: 0,
                current_component: $current_comp,
                active_entities: $active_entities,
                entity_layers: $entity_layers,
                $($out)*
                [<$comp:snake>]: match unsafe {foundry::ComponentTable::get_component_array_mut::<$comp>($components)} {
                    Some(comp_arr) => comp_arr.iter_mut().peekable(),
                    None => [].iter_mut().peekable(),
                },
            }
        }
    };
}

/// WARNING INNER MACRO : do not call by yourself.
/// Create the result query for the iteration.
/// The unsafe borrow are ok beecause we asserted that the components were different,
/// thus we are borrowing different parts of the table.
#[macro_export]
macro_rules! create_result {
    ($components:expr, $ent_mask:expr, $($comps:tt)+) => {
        // call the result inner gen
        {
            use foundry::create_result_inner;
            let current_comp = macro_generated_reset();
            let active_entities = foundry::ComponentTable::get_active_entities($components);
            let entity_layers = foundry::ComponentTable::get_layers($components);
            create_result_inner!(
                $components;
                $ent_mask;
                MacroGeneratedComponentIterator {
                    entity_mask: 0,
                    current_entity: 0,
                    current_component: current_comp,
                    active_entities: active_entities,
                    entity_layers: entity_layers,
                    ()
                }, $($comps)+
            )
        }
    };
}