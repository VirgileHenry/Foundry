
#[macro_export]
macro_rules! get_components_inner {
    ($components:expr; $entity:expr; ($($out:tt)*); mut $comp:ident, $($rest:tt)+) => {
        foundry::get_components_inner!(
            $components;
            $entity;
            ($($out)* unsafe {foundry::ComponentTable::unsafe_get_comp_mut::<$comp>($components, $entity)},);
            $($rest)*
        )
    };
    ($components:expr; $entity:expr; ($($out:tt)*); $comp:ident, $($rest:tt)+) => {
        foundry::get_components_inner!(
            $components;
            $entity;
            ($($out)* foundry::ComponentTable::get_component::<$comp>($components, $entity),);
            $($rest)*
        )
    };
    ($components:expr; $entity:expr; ($($out:tt)*); mut $comp:ident) => {
        ($($out)* unsafe {foundry::ComponentTable::unsafe_get_comp_mut::<$comp>($components, $entity)})
    };
    ($components:expr; $entity:expr; ($($out:tt)*); $comp:ident) => {
        ($($out)* foundry::ComponentTable::get_component::<$comp>($components, $entity))
    };
}

/// Get multiple components on a single entity, in a non mutable or mutable way.
#[macro_export]
macro_rules! get_components {
    ($components:expr; $entity:expr; $($comps:tt)+) => {
        {
            // static assert that we are asking different components, so it stays safe !
            // this prevents asking for same component multiple times mutably
            assert_exclusive_types!($($comps)+);
            // todo : multiple entities as well ?
            let out = foundry::get_components_inner!($components; $entity; (); $($comps)+);
            out // todo : macro to convert tuple of option to option of tuples
        }
    }
}

