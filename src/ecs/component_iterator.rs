pub use crate::{
    generate_query,
    generate_query_mut,
    impl_query,
    impl_query_mut,
    FoundryBoolVecInner,
    FoundryEntityMasks,
    FoundryIndexedElemInner,
};

pub(crate) mod query;
pub(crate) mod query_mut;
pub(crate) mod query_state;


generate_query!(Query1d, Query1dState, T1);
generate_query!(Query2d, Query2dState, T1, T2);
generate_query!(Query3d, Query3dState, T1, T2, T3);
generate_query!(Query4d, Query4dState, T1, T2, T3, T4);
generate_query!(Query5d, Query5dState, T1, T2, T3, T4, T5);
generate_query!(Query6d, Query6dState, T1, T2, T3, T4, T5, T6);
generate_query!(Query7d, Query7dState, T1, T2, T3, T4, T5, T6, T7);
generate_query!(Query8d, Query8dState, T1, T2, T3, T4, T5, T6, T7, T8);

impl_query!(Query1d, Query1dState, T1);
impl_query!(Query2d, Query2dState, T1, T2);
impl_query!(Query3d, Query3dState, T1, T2, T3);

generate_query_mut!(Query1dMut, Query1dMutState, T1);
generate_query_mut!(Query2dMut, Query2dMutState, T1, T2);
generate_query_mut!(Query3dMut, Query3dMutState, T1, T2, T3);
generate_query_mut!(Query4dMut, Query4dMutState, T1, T2, T3, T4);
generate_query_mut!(Query5dMut, Query5dMutState, T1, T2, T3, T4, T5);
generate_query_mut!(Query6dMut, Query6dMutState, T1, T2, T3, T4, T5, T6);
generate_query_mut!(Query7dMut, Query7dMutState, T1, T2, T3, T4, T5, T6, T7);
generate_query_mut!(Query8dMut, Query8dMutState, T1, T2, T3, T4, T5, T6, T7, T8);

impl_query_mut!(Query1dMut, Query1dMutState, T1);
impl_query_mut!(Query2dMut, Query2dMutState, T1, T2);
impl_query_mut!(Query3dMut, Query3dMutState, T1, T2, T3);