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

