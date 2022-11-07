
/// entity is not owned by the world, more like a wrapper to ease interface with the component table.
/// This is more a entityRef than an entity, copying it will not duplicate entities but duplicate the ref.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EntityRef {
    pub id: usize,
}
