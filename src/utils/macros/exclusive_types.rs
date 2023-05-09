

#[macro_export]
macro_rules! assert_exclusive_types_inner {
    ($exc_trait:tt; mut $comp:ident, $($rest:tt)+) => {
        {
            impl $exc_trait for $comp {}
            assert_exclusive_types_inner!($exc_trait; $($rest)+)
        }
    };
    ($exc_trait:tt; $comp:ident, $($rest:tt)+) => {
        {
            impl $exc_trait for $comp {}
            assert_exclusive_types_inner!($exc_trait; $($rest)+)
        }
    };
    ($exc_trait:ty; mut $comp:ident) => {
        { impl $exc_trait for $comp {} }
    };
    ($exc_trait:ty; $comp:ident) => {
        { impl $exc_trait for $comp {} }
    };
}

/// Implements the given trait for all types, wether they are preceeded with the 'mut' keyword or not.
#[macro_export]
macro_rules! assert_exclusive_types {
    ($($comps:tt)*) => {
        {
            // inspired from the `static_assertions` crate, but had to be rework to match our case.
            // the idea is to impl a trait for all given types, and the compiler will throw a conflicting impl if same type appear twice.
            trait MutuallyExclusive {}
            assert_exclusive_types_inner!(MutuallyExclusive; $($comps)*);
        }
    };
}