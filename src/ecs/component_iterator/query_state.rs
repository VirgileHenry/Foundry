#[macro_export]
macro_rules! query_state_funcs {
    ($name:ty, $first:path, $($elems:path),*) => {
        impl $name {
            #[inline]
            pub fn next_state(self) -> Self {
                match self {
                    $first => 
                    $($elems, $elems =>)*
                    $first
                }
            }
    
            #[inline]
            pub fn reset_state() -> Self {
                $first
            }
        }
    }
}