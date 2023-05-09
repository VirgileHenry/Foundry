#[macro_export]
macro_rules! create_result_inner {
    (
        $components:expr;
        MacroGeneratedComponentIterator {
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            ($($out:tt)*)
        }, $comp:ident, $($rest:tt)*
    ) => {
        paste::paste! {
            create_result_inner!(
                $components;
                MacroGeneratedComponentIterator {
                    current_entity: 0,
                    current_component: $current_comp,
                    active_entities: $active_entities,
                    (
                        $($out)*
                        [<$comp:snake>]: match foundry::ComponentTable::get_component_array::<$comp>($components) {
                            Some(comp_arr) => comp_arr.iter().peekable(),
                            None => [].iter().peekable(),
                        },
                    )
                }, $($rest)*
            )
        }
    };
    (
        $components:expr;
        MacroGeneratedComponentIterator {
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            ($($out:tt)*)
        }, $comp:ident
    ) => {
        paste::paste! {
            MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component: $current_comp,
                active_entities: $active_entities,
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
        MacroGeneratedComponentIterator {
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            ($($out:tt)*)
        }, mut $comp:ident, $($rest:tt)*
    ) => {
        paste::paste! {
            create_result_inner!(
                $components;
                MacroGeneratedComponentIterator {
                    current_entity: 0,
                    current_component: $current_comp,
                    active_entities: $active_entities,
                    (
                        $($out:tt)*
                        [<$comp:snake>]: match unsafe {foundry::ComponentTable::get_component_array_mut::<$comp>($components)} {
                            Some(comp_arr) => comp_arr.iter_mut().peekable(),
                            None => [].iter_mut().peekable(),
                        },
                    )
                }, $($rest)*
            )
        }
    };
    (
        $components:expr;
        MacroGeneratedComponentIterator {
            current_entity: 0,
            current_component: $current_comp:expr,
            active_entities: $active_entities:expr,
            ($($out:tt)*)
        }, mut $comp:ident
    ) => {
        paste::paste! {
            MacroGeneratedComponentIterator {
                current_entity: 0,
                current_component: $current_comp,
                active_entities: $active_entities,
                $($out)*
                [<$comp:snake>]: match unsafe {foundry::ComponentTable::get_component_array_mut::<$comp>($components)} {
                    Some(comp_arr) => comp_arr.iter_mut().peekable(),
                    None => [].iter_mut().peekable(),
                },
            }
        }
    };
}

#[macro_export]
macro_rules! create_result {
    ($components:expr, $($comps:tt)+) => {
        // call the result inner gen
        {
            use foundry::create_result_inner;
            let current_comp = macro_generated_reset();
            let active_entities = foundry::ComponentTable::get_active_entities($components);
            create_result_inner!(
                $components;
                MacroGeneratedComponentIterator {
                    current_entity: 0,
                    current_component: current_comp,
                    active_entities: active_entities,
                    ()
                }, $($comps)*
            )
        }
    };
}