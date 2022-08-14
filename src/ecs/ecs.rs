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
    
    pub fn create_entity_with_1<C: 'static>(&mut self, component: C) -> Entity {
        // C is a component
        let result: Entity = Entity {
            id: self.last_entity_id,
        };
        self.last_entity_id += 1;
        // add the component, as we just created the entity we can fast push in the packed array
        self.components.add_comp_to_last(&result, component);
        return result;
    }

    pub fn create_entity_with_2<C1: 'static, C2: 'static>(&mut self, component_1: C1, component_2: C2) -> Entity {
        // C is a component
        let result: Entity = Entity {
            id: self.last_entity_id,
        };
        self.last_entity_id += 1;
        // add the component, as we just created the entity we can fast push in the packed array
        self.components.add_comp_to_last(&result, component_1);
        self.components.add_comp_to_last(&result, component_2);

        return result;
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        // how to know what components this entity possess ?
        // todo : either entity store components ref, or look for every component
    }

    // todo : ways to create entity with components, and multiple entities with same components types

    pub fn add_component<C: 'static>(&mut self, entity: &Entity, component: C) -> Option<C> {
        return self.components.add_component(entity, component);
    }

    pub fn get_component<C: 'static>(&mut self, entity: &Entity) -> Option<&C> {
        return self.components.get_component::<C>(entity);
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
            system.elem.update(&self.components, delta);
        }
    }
}

