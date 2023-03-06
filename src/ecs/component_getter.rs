
#[macro_export]
macro_rules! get_components_impl {
    ($components:expr; $entity:expr; ($($out:tt)*); mut $comp:ident, $($rest:tt)+) => {
        get_components_impl!(
            $components;
            $entity;
            ($($out)* unsafe {foundry::ComponentTable::unsafe_get_comp_mut::<$comp>($components, $entity)},);
            $($rest)*
        )
    };
    ($components:expr; $entity:expr; ($($out:tt)*); $comp:ident, $($rest:tt)+) => {
        get_components_impl!(
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


#[macro_export]
macro_rules! get_components {
    ($components:expr; $entity:expr; $($comp:tt)+) => {
        {
            let out = get_components_impl!($components; $entity; (); $($comp)+);
            out // todo : macro to convert tuple of option to option of tuples
        }
    }
}

