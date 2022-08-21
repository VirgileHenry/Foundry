use anymap::AnyMap;

use crate::utils::collections::packed_array::PackedArray;

use super::{
    component_table::ComponentTable,
    system::System,
    entity::{Entity}
};



pub struct ECS {
    // can be considerer to be a world
    // all the components on the entities
    pub components: ComponentTable, // todo : private
    // all the systems, ids being order of execution
    systems: PackedArray<System>,
    last_entity_id: usize,
}

impl ECS {
    pub fn new() -> ECS {
        return ECS {  
            components: ComponentTable::new(),
            systems: PackedArray::new(),
            last_entity_id: 1,
        };
    }

    pub fn create_entity(&mut self) -> Entity {
        let result: Entity = Entity {
            id: self.last_entity_id,
        };
        self.last_entity_id += 1;
        return result;
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        todo!();
    }

    pub fn create_entities(&mut self, count: usize) -> Vec<Entity> {
        let mut result = Vec::with_capacity(count);
        for i in 0..count {
            result.push(Entity { id: self.last_entity_id + i });
        }
        self.last_entity_id += count;
        return result;
    }
    
    pub fn get_unsafe_component_map(&self) -> &anymap::Map {
        return self.components.get_component_map();
    }

    pub fn add_component<C: 'static>(&mut self, entity: &Entity, component: C) -> Option<C> {
        return self.components.add_component(entity, component);
    }

    pub fn get_component<C: 'static>(&mut self, entity: &Entity) -> Option<&C> {
        return self.components.get_component::<C>(entity);
    }

    pub fn get_component_mut<C: 'static>(&mut self, entity: &Entity) -> Option<&mut C> {
        return self.components.get_component_mut::<C>(entity);
    }

    pub fn remove_component<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        return self.components.remove_component::<C>(entity);
    }

    pub fn register_system(&mut self, system: System, index: usize) {
        self.systems.insert(system, index);
    }

    pub fn update(&mut self, delta: f32) {
        // update every system in order
        for system in self.systems.iter_mut() {
            system.elem.update(&mut self.components, delta);
        }
    }
}

// macros to create entities with any number of components
#[macro_export]
macro_rules! create_entity {
    ($ecs:expr) => { ECS::create_entity(&mut $ecs) };
    ($ecs:expr; $($comp:expr),*) => { {
        let result_entity = ECS::create_entity(&mut $ecs);
        $(
            $ecs.components.add_comp_to_last(&result_entity, $comp);
        )*
        result_entity
    } };
}

#[macro_export]
macro_rules! create_entities {
    ($ecs:expr; $amount:expr, $($generators:expr),*) => {
        {
            let result_entities = ECS::create_entities(&mut $ecs, $amount);
            let start_index = match result_entities.get(0) {Some(entity) => entity.id, None => 0};
            $(
                let mut comp_vec = Vec::with_capacity($amount);
                for i in 0..$amount {
                    comp_vec.push($generators(i));
                }
                $ecs.components.add_comps_to_last(start_index, comp_vec);
            )*
            result_entities
        }
    };
}