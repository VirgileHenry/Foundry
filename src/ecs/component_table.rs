extern crate anymap;
use crate::{ecs::{
    entity::Entity,
    component_array::ComponentArray,
}, utils::collections::{packed_array::IndexedElem, bool_vec::BoolVec}};

/// Stores all the components of the entites packed up together to ease iteration.
pub struct ComponentTable {
    /// Anymap of the components where keys are component types, values are components arrays.
    components: anymap::Map,
    /// Vec keeping track of all the active entities.
    active_entities: BoolVec,
    /// the count of how many entities have been created, does not count the deleted ones.
    entity_count: usize,
}

/// Struct keeping all the components with methods to add, remove, create entities, etc...
impl ComponentTable {
    /// Create a new empty component table
    pub fn new() -> ComponentTable {
        return ComponentTable {
            components: anymap::Map::new(),
            active_entities: BoolVec::new(),
            entity_count: 0,
        };
    }

    /// Create a new entity and return it.
    pub fn create_entity(&mut self) -> Entity {
        let result = Entity {
            id: self.entity_count
        };
        self.active_entities.push(true);
        self.entity_count += 1;
        result
    }

    /// Destroy an entity. This is not implmented as it only deactivate it and does not delete its components in memory.
    pub fn destroy_entity(&mut self, entity: Entity) {
        // this is bad, we move the entity and deactivate it.
        // need to find a good way to remove all it's components to clean memory
        self.set_entity_active(&entity, false);
    }

    /// Create multiple entities at once.
    /// It's more efficient than calling ```create_entity``` multiple times.
    pub fn create_entities(&mut self, count: usize) -> Vec<Entity> {
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            result.push(Entity { id: self.entity_count + i });
        }
        self.active_entities.append(BoolVec::all_true(count));
        self.entity_count += count;
        result
    }

    /// Set an entity as active or not.
    /// Inactive entities still exists, but are ignored by iterators over components and are not updated.
    pub fn set_entity_active(&mut self, entity: &Entity, active: bool) {
        self.active_entities.set(entity.id, active);
    }

    /// Add the given component to the given entity. 
    /// If the entity already had this type of component, it is replaced and returned. Otherwise, None is returned.
    pub fn add_component<C: 'static>(&mut self, entity: &Entity, component: C) -> Option<C> {
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => {
                // case where the component array exist
                return components.insert_component(component, entity.id);
            },
            None => {
                // create the component array from the new element
                self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_elem(component, entity.id));
                return None;
            },
        };
    }

    /// Adds a component to an entity assuming no entity with a highter id have this time of component.
    /// This is faster than ```add_component``` but can break the table if the condition isn't valid.
    /// This is only used when creating an entity with components.
    pub fn add_comp_to_last<C: 'static>(&mut self, entity: &Entity, component: C) {
        // this may only be used if we are ensured the entity is the last one, and it does not have this component yet
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => components.append_component(component, entity.id),
            None => { self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_elem(component, entity.id)); },
        };
    }

    /// Adds a vec of components to a range of entities assuming no entity with a highter id have this time of component.
    /// This is faster than ```add_component``` but can break the table if the condition isn't valid.
    /// This is only used when creating entities with components.
    pub fn add_comps_to_last<C: 'static>(&mut self, start_index: usize, component_vec: Vec<C>) {
        match self.components.get_mut::<ComponentArray<C>>() {
            Some(components) => components.append_components(component_vec, start_index),
            None => { self.components.insert::<ComponentArray<C>>(ComponentArray::<C>::new_with_vec(component_vec, start_index));},
        };
    }

    /// Access to the raw component array of components. 
    pub fn get_component_array<C: 'static>(&self) -> Option<&Vec<IndexedElem<C>>> {
        return match self.components.get::<ComponentArray<C>>() {
            Some(comp_arr) => Some(comp_arr.get_array()),
            None => None,
        }
    }

    /// Mutable access to the raw array of components.
    pub fn get_component_array_mut<C: 'static>(&self) -> Option<&mut Vec<IndexedElem<C>>> {
        return match self.components.get::<ComponentArray<C>>() {
            Some(comp_arr) => Some(comp_arr.unsafe_get_array_mut()),
            None => None,
        }
    }

    /// Get a reference to a component of the given type of an entity.
    pub fn get_component<C: 'static>(&self, entity: &Entity) -> Option<&C> {
        return match self.components.get::<ComponentArray<C>>() {
            None => None,
            Some(comp_arr) => comp_arr.get_component(entity.id),
        }
    }

    /// Get a mutable reference to a component of the given type of an entity.
    pub fn get_component_mut<C: 'static>(&mut self, entity: &Entity) -> Option<&mut C> {
        return match self.components.get_mut::<ComponentArray<C>>() {
            None => None,
            Some(comp_arr) => comp_arr.get_component_mut(entity.id),
        }
    }

    /// Get a reference to the vec containing the active entities.
    pub fn get_active_entities(&self) -> &BoolVec {
        return &self.active_entities;
    }

    /// Removes a component of the given type of an entity, and return it if there was any.
    pub fn remove_component<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        return match self.components.get_mut::<ComponentArray<C>>() {
            None => None,
            Some(comp_arr) => comp_arr.remove_component(entity.id),
        }
    }

}

/// Creates an entity with any number of components.
/// This is way faster than ```create_entity``` then ```add_component```
/// This can be used passing the component table, then the components for exemple : 
/// ```let entity = create_entity!(comp_table; Position{x:0, y:0}, Velocity{vx:0, vy:0});```
#[macro_export]
macro_rules! create_entity {
    ($comp_table:expr) => { 
        use foundry::ecs::component_table::ComponentTable;
        ComponentTable::create_entity(&mut $comp_table)
    };
    ($comp_table:expr; $($comp:expr),*) => { {
        use foundry::ecs::component_table::ComponentTable;

        let result_entity = ComponentTable::create_entity(&mut $comp_table);
        $(
            $comp_table.add_comp_to_last(&result_entity, $comp);
        )*
        result_entity
    } };
}

/// Creates multiple entities with components.
/// This is way faster than creating entities and adding components to each and everyone of them
/// Uses generators functions for the components allowing to give unique values to the components of the created entities.
/// for example : ```let entities = creates_entities!(comp_table; 1000, |i:usize|{Position{x:i, y:i}});```
#[macro_export]
macro_rules! create_entities {
    ($comp_table:expr; $amount:expr, $($generators:expr),*) => {
        {
            use foundry::ecs::component_table::ComponentTable;
            let result_entities = ComponentTable::create_entities(&mut $comp_table, $amount);
            let start_index = match result_entities.get(0) {Some(entity) => entity.id, None => 0};
            $(
                let mut comp_vec = Vec::with_capacity($amount);
                for i in 0..$amount {
                    comp_vec.push($generators(i));
                }
                $comp_table.add_comps_to_last(start_index, comp_vec);
            )*
            result_entities
        }
    };
}