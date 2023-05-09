
/// Entities are an index that reference components.
/// Cloning an entity will only clone the reference, not the entity itself.
/// todo : maybe this should be called EntityRef ?
pub type Entity = usize;