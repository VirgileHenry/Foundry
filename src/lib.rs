pub extern crate paste;

pub(crate) mod ecs;
pub(crate) mod utils;

#[cfg(test)]
pub(crate) mod test;

// what we export from our lib
pub use ecs::{
    component_table::ComponentTable,
    world::World,
    system::{
        AsAny,
        System,
        Updatable,
        UpdateFrequency,
    },
};

// expose our proc macros
pub use foundry_macro::AsAny;

// create public types but would be better to hide them
// however, they are needed in a macro so I don't know how to hide them
pub type FoundryBoolVecInner = utils::collections::bool_vec::BoolVec;
pub type FoundryIndexedElemInner<T> = utils::collections::packed_array::IndexedElem<T>;
pub type FoundryEntityMasks = Vec<u32>;