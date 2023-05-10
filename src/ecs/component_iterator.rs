pub(crate) mod create_result;
pub(crate) mod enum_helper_funcs;
pub(crate) mod generate_enum;
pub(crate) mod generate_result_struct;
pub(crate) mod impl_iterator;

#[macro_export]
macro_rules! component_iterator {
    ($components:expr $(, $mask:expr)?; $($comps:tt)+) => {
        {
            // create the entity mask : u32::MAX if not given.
            // hopefully, this little trick would get easily optimized by the compiler
            let mut ent_mask = u32::MAX;
            $(
                // if we got a mask value, replace the max by it
                ent_mask = $mask;
            )?
            // static assert that the types are mutually exclusive
            use foundry::paste;
            use foundry::assert_exclusive_types;
            use foundry::assert_exclusive_types_inner;
            assert_exclusive_types!($($comps)+);
            // generate the enum used for the iterator
            use foundry::generate_enum;
            use foundry::generate_enum_inner;
            generate_enum!($($comps)+);
            // generate the result struct with correct mutability
            use foundry::generate_result_struct;
            use foundry::generate_result_struct_impl;
            generate_result_struct!($($comps)+);
            // impl iterator for our result struct
            use foundry::impl_iterator;
            use foundry::impl_iterator_inner;
            impl_iterator!($($comps)+);
            // finally, create an instance of the result struct
            use foundry::create_result;
            use foundry::create_result_inner;
            create_result!($components, ent_mask, $($comps)+)
        }
    };
}

/*
Work in progress : this whole iteration macro could and should (maybe will ?) be optimized.
Comparing to bevy or legion, we are actually 10x slower to iterate over entities.
I think this is due to the structure, but also to the mask and entity activity that add lots of checks.
 */