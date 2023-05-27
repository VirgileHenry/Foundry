/// WARNING INNER MACRO : do not call by yourself.
/// Inner implementation of the macro to generate the enum.
/// Filters out the 'mut' keywords with recursive calls.
#[macro_export]
macro_rules! generate_enum_inner {
    (($($out:tt)*); mut $comp:ident, $($rest:tt)+) => {
        generate_enum_inner!(($($out)* $comp,); $($rest)+)
    };
    (($($out:tt)*); $comp:ident, $($rest:tt)+) => {
        generate_enum_inner!(($($out)* $comp,); $($rest)+)
    };
    (($($out:tt)*); mut $comp:ident) => {
        generate_enum_inner!(@end $($out)* $comp)
    };
    (($($out:tt)*); $comp:ident) => {
        generate_enum_inner!(@end $($out)* $comp)
    };
    (@end $($comp:ident),*) => {
        paste::paste! {
            #[derive(Copy, Clone)]
            enum MacroGeneratedComponentsEnum {
                $(
                    [<$comp:camel>],
                )+
                EndOfIterator
            }
            // also, let's create the enum helper funcs
            foundry::enum_helper_funcs!(
                MacroGeneratedComponentsEnum;
                $(MacroGeneratedComponentsEnum::[<$comp:camel>],)+
                MacroGeneratedComponentsEnum::EndOfIterator
            );
        }
    };
}

/// WARNING INNER MACRO : do not call by yourself.
/// Generate the enum iterator that allows to iterate over different component types.
/// We need a separate macro to filter out the 'mut' we get in the input.
#[macro_export]
macro_rules! generate_enum {
    ($($comps:tt)*) => {
        generate_enum_inner!((); $($comps)*)
    };
}
