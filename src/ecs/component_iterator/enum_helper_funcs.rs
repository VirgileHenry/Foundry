
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
