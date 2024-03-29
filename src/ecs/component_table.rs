use crate::{ecs::{
    entity::Entity,
    component_array::ComponentArray,
}, utils::collections::{packed_array::IndexedElem, bool_vec::BoolVec}};

/// Stores all the components of the entites packed up together to ease iteration.
pub struct ComponentTable {
    /// Anymap of the components where keys are component types, values are components arrays.
    components: anymap::AnyMap,
    /// singleton components are components that exists outside of entities, and there are only one instance of each type.
    singletons: anymap::AnyMap,
    /// Vec keeping track of all the active entities.
    active_entities: BoolVec,
    /// Layers masks on entities
    entity_layers: Vec<u32>,
    /// the count of how many entities have been created, does not count the deleted ones.
    entity_count: usize,
}

impl Default for ComponentTable {
    /// Creates a new, empty component table.
    fn default() -> Self {
        Self::new()
    }
}

/// Struct keeping all the components with methods to add, remove, create entities, etc...
impl ComponentTable {
    /// Create a new empty component table
    pub fn new() -> ComponentTable {
        return ComponentTable {
            components: anymap::Map::new(),
            singletons: anymap::Map::new(),
            active_entities: BoolVec::new(),
            entity_layers: Vec::new(),
            entity_count: 0,
        };
    }

    /// Create a new entity and return it.
    pub fn create_entity(&mut self) -> Entity {
        let result = self.entity_count;
        self.active_entities.push(true);
        self.entity_layers.push(u32::MAX);
        self.entity_count += 1;
        result
    }

    /// Destroy an entity. This is not implemented as it only deactivate it and does not delete its components in memory.
    pub fn destroy_entity(&mut self, entity: Entity) {
        // this is bad, we move the entity and deactivate it.
        // but it still can be accessed with manual entity ref contruction !
        // need to find a good way to remove all it's components to clean memory
        self.set_entity_active(entity, false);
    }

    /// Create multiple entities at once.
    /// It's more efficient than calling ```create_entity``` multiple times.
    pub fn create_entities(&mut self, count: usize) -> Vec<Entity> {
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            result.push(self.entity_count + i);
        }
        self.active_entities.append(BoolVec::all_true(count));
        self.entity_layers.append(&mut vec![u32::MAX; count]);
        self.entity_count += count;
        result
    }

    /// Set an entity as active or not.
    /// Inactive entities still exists, but are ignored by iterators over components and are not updated.
    #[inline]
    pub fn set_entity_active(&mut self, entity: Entity, active: bool) {
        self.active_entities.set(entity, active);
    }

    /// Tells if an entity is active or not. Returns None if the entity is not found.
    #[inline]
    pub fn is_entity_active(&self, entity: Entity) -> Option<bool> {
        self.active_entities.get(entity)
    }

    /// Add the given component to the given entity. 
    /// If the entity already had this type of component, it is replaced and returned. Otherwise, None is returned.
    pub fn add_component<C: 'static>(&mut self, entity: Entity, component: C) -> Option<C> {
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => components.insert_component(component, entity),
            None => {
                // component array does not exist : create the component array from the new element
                self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_elem(component, entity));
                None
            },
        }
    }

    /// add the given singleton component to the table.
    #[inline]
    pub fn add_singleton<C: 'static>(&mut self, component: C) -> Option<C> {
        self.singletons.insert(component)
    }

    /// get a reference to the asked singleton component.
    #[inline]
    pub fn get_singleton<C: 'static>(&self) -> Option<&C> {
        self.singletons.get::<C>()
    }

    /// get a mutable reference to the asked singleton component.
    #[inline]
    pub fn get_singleton_mut<C: 'static>(&mut self) -> Option<&mut C> {
        self.singletons.get_mut::<C>()
    }

    #[inline]
    pub fn remove_singleton<C: 'static>(&mut self) -> Option<C> {
        self.singletons.remove::<C>()
    }

    /// Adds a component to an entity assuming no entity with a highter id have this time of component.
    /// This is faster than ```add_component``` but can break the table if the condition isn't valid.
    /// This should only be used when creating an entity with components.
    pub fn add_comp_to_last<C: 'static>(&mut self, entity: Entity, component: C) {
        // this may only be used if we are ensured the entity is the last one, and it does not have this component yet
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => components.append_component(component, entity),
            None => { self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_elem(component, entity)); },
        };
    }

    /// Adds a vec of components to a range of entities assuming no entity with a highter id have this time of component.
    /// This is faster than ```add_component``` but can break the table if the condition isn't valid.
    /// This should only be used when creating entities with components.
    pub fn add_comps_to_last<C: 'static>(&mut self, start_index: usize, component_vec: Vec<C>) {
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => components.append_components(component_vec, start_index),
            None => { self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_vec(component_vec, start_index));},
        };
    }

    /// Access to the raw component array of components. 
    #[inline]
    pub fn get_component_array<C: 'static>(&self) -> Option<&Vec<IndexedElem<C>>> {
        Some(self.components.get::<ComponentArray<C>>()?.get_array())
    }

    /// Mutable access to the raw array of components.
    #[inline]
    pub unsafe fn get_component_array_mut<C: 'static>(&self) -> Option<&mut Vec<IndexedElem<C>>> {
        Some(self.components.get::<ComponentArray<C>>()?.unsafe_get_array_mut())
    }

    /// Get a reference to a component of the given type of an entity.
    #[inline]
    pub fn get_component<C: 'static>(&self, entity: Entity) -> Option<&C> {
        return self.components.get::<ComponentArray<C>>()?.get_component(entity)
    }

    /// Get a mutable reference to a component of the given type of an entity.
    #[inline]
    pub fn get_component_mut<C: 'static>(&mut self, entity: Entity) -> Option<&mut C> {
        self.components.get_mut::<ComponentArray<C>>()?.get_component_mut(entity)
    }

    /// get a mutable access to a component without borrowing the table mutably.
    #[inline]
    pub unsafe fn unsafe_get_comp_mut<C: 'static>(&self, entity: Entity) -> Option<&mut C> {
        self.components.get::<ComponentArray<C>>() ?.unsafe_get_comp_mut(entity)
    } 

    /// Get a reference to the vec containing the active entities.
    #[inline]
    pub fn get_active_entities(&self) -> &BoolVec {
        return &self.active_entities;
    }

    /// Removes a component of the given type of an entity, and return it if there was any.
    #[inline]
    pub fn remove_component<C: 'static>(&mut self, entity: Entity) -> Option<C> {
        self.components.get_mut::<ComponentArray<C>>()?.remove_component(entity)
    }

    /// check if the entity have at least one layer in common with the given mask
    #[inline]
    pub fn test_entity_layers(&self, entity: Entity, mask: u32) -> Option<bool> {
        Some((self.entity_layers.get(entity)? & mask) > 0)
    }

    /// check if a specific layer of the entity is active.
    #[inline]
    pub fn test_entity_layer(&self, entity: Entity, layer: u8) -> Option<bool> {
        Some((self.entity_layers.get(entity)? & (1 << layer)) > 0)
    }

    /// set an entity's layers.
    #[inline]
    pub fn set_entity_layers(&mut self, entity: Entity, layers: u32) {
        match self.entity_layers.get_mut(entity) {
            Some(v) => *v = layers,
            None => {},
        };
    }

    /// get the entity layers.
    #[inline]
    pub fn get_entity_layers(&self) -> &Vec<u32> {
        &self.entity_layers
    }


    /// set an entity single layer.
    pub fn set_entity_layer(&mut self, entity: Entity, layer: u8, value: bool) {
        match self.entity_layers.get_mut(entity) {
            Some(layers) => if value {
                // set the bit
                *layers |= 1 << layer;
            } else {
                // unset the bit
                *layers &= !(1 << layer);
            }
            None => {},
        }
    }

    /// flips all the layers of a given entity.
    pub fn toggle_entity_layers(&mut self, entity: Entity) {
        match self.entity_layers.get_mut(entity) {
            Some(layers) => *layers = !*layers,
            None => {},
        }
    }

    /// flip the given layer of a given entity.
    pub fn toggle_entity_layer(&mut self, entity: Entity, layer: u8) {
        match self.entity_layers.get_mut(entity) {
            Some(layers) => *layers ^= 1 << layer,
            None => {},
        }
    }

    /// Remove an entire row of components from the comp table, returning an iterator over it.
    pub fn drain_components<C: 'static>(&mut self) -> Option<impl Iterator<Item = (usize, C)>> {
        Some(self.components.remove::<ComponentArray<C>>()?.drain())
    }

}
