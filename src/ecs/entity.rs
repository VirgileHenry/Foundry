
// entity is not owned by the world, more like a wrapper to ease interface with the component table
// This is more a entityRef than an entity, copying it will not duplicate entities
#[derive(Clone, Copy)]
pub struct Entity {
    pub id: usize,
}