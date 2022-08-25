mod ecs;
use ecs::{
    entity::Entity,
    system::{
        Updatable, 
        System,
        UpdateFrequency
    }
};

use crate::ecs::ecs::ECS;
mod utils;



struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    vx: f32,
    vy: f32,
}

#[test]
fn component_test() {
    let mut ecs: ECS = ECS::new();
    let entity: Entity = create_entity!(ecs);
    let entity1: Entity = create_entity!(ecs; Position{x:1.0, y:0.5});
    let entity2: Entity = create_entity!(ecs; Position{x: 0.2, y: 1.3}, Velocity{vx:0.1, vy:-0.3});
    match ecs.get_component_mut::<Position>(&entity1) {
        None => println!("Unable to find component position"),
        Some(pos) => println!("Found position at {} {}", pos.x, pos.y),
    }
}

#[test]
fn entity_macro_creation_test() {
    let mut ecs = ECS::new();
    // objective :
    // let entity = create_entity!(ecs, Position{...}, Velocity{...});
}

#[test]
fn iterate_component_test() {
    let mut ecs = ECS::new();
    let mut entities: Vec<Entity> = create_entities!(ecs; 100, |i:usize| -> Position {Position{x:i as f32, y:i as f32}} );

    for component in iterate_over_component!(&mut ecs; Position) {
        println!("reading positions : {} {}", component.x, component.y);
    }

    for component in iterate_over_component!(&mut ecs; Velocity) {
        println!("reading velocity : {} {}", component.vx, component.vy);
    }

    for component in iterate_over_component!(&mut ecs; Velocity) {
        println!("writing velocity : {} {}", component.vx, component.vy);
    }

    for comps in iterate_over_component!(&mut ecs; Position, Velocity) {
        let (pos, vel) = comps; // unpack
        println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
    }

    ecs = ECS::new();
    entities = vec!();
    for i in 0..100 {
        if i % 2 == 0 {
            entities.push(create_entity!(ecs; Position { x: i as f32, y: (100 - i) as f32 }));
        }
        else {
            entities.push(create_entity!(ecs; Velocity { vx: i as f32, vy: (100 - i) as f32 }));
        }
    }
    println!("Should found no entity with both components :");
    
    for comps in iterate_over_component!(&mut ecs; Position, Velocity) {
        let (pos, vel) = comps; // unpack
        println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
    }

    ecs = ECS::new();
    entities = vec!();
    for i in 0..100 {
        if i > 50 {
            entities.push(create_entity!(ecs; Position { x: i as f32, y: (100 - i) as f32 }));
        }
        else if i < 50 {
            entities.push(create_entity!(ecs; Velocity { vx: i as f32, vy: (100 - i) as f32 }));
        }
        else {
            entities.push(create_entity!(ecs; Velocity { vx: i as f32, vy: (100 - i) as f32 }, Position { x: i as f32, y: (100 - i) as f32 }));
        }
    }
    println!("Should found one entity with both components :");
    
    for comps in iterate_over_component!(&mut ecs; Position, Velocity) {
        let (pos, vel) = comps; // unpack
        println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
    }

}


struct PhysicSystem {
    gravity_x: f32,
    gravity_y: f32,
}

impl Updatable for PhysicSystem {
    fn update(&mut self, components: &mut ecs::component_table::ComponentTable, delta: f32) {
        for comps in iterate_over_component_from_sys!(components; Position, Velocity) {
            let (pos, vel) = comps; // unpack
            println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
            // try iterate over each position
            
        }
    }
}


#[test] 
fn system_test() {
    let physics: PhysicSystem = PhysicSystem { gravity_x: 0.0, gravity_y: -9.81 };
    let physic_sys = System::new(Box::new(physics), UpdateFrequency::Fixed(0.05));
    let mut ecs = ECS::new();
    ecs.register_system(physic_sys, 0);
}


fn main() {

    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        // create ecs and entities
        let mut ecs = ECS::new();
        let mut entities: Vec<Entity> = create_entities!(ecs; 1_000,
            |i:usize| -> Position {Position{x:i as f32, y:i as f32}},
            |i:usize| -> Velocity {Velocity { vx: i as f32, vy: i as f32 }} );

        // let's debug this
        for (pos, vel) in {
            let mut comp_map = ECS::get_unsafe_component_map(&ecs);
            {
                use crate::utils::collections::packed_array::IndexedElem;
                use crate::ecs::component_array::ComponentArray;
                enum MacroGeneratedComponentsEnum {
                    Position,
                    Velocity,
                    EndOfIterator,
                }
                #[automatically_derived]
                impl ::core::marker::Copy for MacroGeneratedComponentsEnum {}
                #[automatically_derived]
                impl ::core::clone::Clone for MacroGeneratedComponentsEnum {
                    #[inline]
                    fn clone(&self) -> MacroGeneratedComponentsEnum {
                        *self
                    }
                }
                fn macro_generated_return_next(
                    elem: MacroGeneratedComponentsEnum,
                ) -> MacroGeneratedComponentsEnum {
                    match elem {
                        MacroGeneratedComponentsEnum::Position => {
                            MacroGeneratedComponentsEnum::Velocity
                        }
                        MacroGeneratedComponentsEnum::Velocity => {
                            MacroGeneratedComponentsEnum::EndOfIterator
                        }
                        MacroGeneratedComponentsEnum::EndOfIterator => {
                            MacroGeneratedComponentsEnum::Position
                        }
                    }
                }
                fn macro_generated_reset() -> MacroGeneratedComponentsEnum {
                    MacroGeneratedComponentsEnum::Position
                }
                struct MacroGeneratedIterableVec<'a, T> {
                    vec: Option<&'a mut Vec<IndexedElem<T>>>,
                    index: usize,
                }
                struct MacroGeneratedComponentIterator<'a, Position, Velocity> {
                    current_iterator: MacroGeneratedComponentsEnum,
                    current_entity: usize,
                    Position: MacroGeneratedIterableVec<'a, Position>,
                    Velocity: MacroGeneratedIterableVec<'a, Velocity>,
                }
                impl<'a, Position, Velocity> Iterator
                for MacroGeneratedComponentIterator<'a, Position, Velocity> {
                    type Item = (&'a mut Position, &'a mut Velocity);
                    fn next(&mut self) -> Option<Self::Item> {
                        let Position: usize = MacroGeneratedComponentsEnum::Position
                            as usize;
                        let Velocity: usize = MacroGeneratedComponentsEnum::Velocity
                            as usize;
                        loop {
                            match self.current_iterator {
                                MacroGeneratedComponentsEnum::Position => {
                                    while match &self.Position.vec {
                                        None => return None,
                                        Some(array) => {
                                            match array.get(self.Position.index) {
                                                None => return None,
                                                Some(i_elem) => {
                                                    if i_elem.index < self.current_entity {
                                                        true
                                                    } else {
                                                        if i_elem.index > self.current_entity {
                                                            self.current_entity = i_elem.index;
                                                            self.current_iterator = macro_generated_reset();
                                                        } else {
                                                            self
                                                                .current_iterator = macro_generated_return_next(
                                                                self.current_iterator,
                                                            );
                                                        }
                                                        false
                                                    }
                                                }
                                            }
                                        }
                                    } {
                                        self.Position.index += 1;
                                    }
                                }
                                MacroGeneratedComponentsEnum::Velocity => {
                                    while match &self.Velocity.vec {
                                        None => return None,
                                        Some(array) => {
                                            match array.get(self.Velocity.index) {
                                                None => return None,
                                                Some(i_elem) => {
                                                    if i_elem.index < self.current_entity {
                                                        true
                                                    } else {
                                                        if i_elem.index > self.current_entity {
                                                            self.current_entity = i_elem.index;
                                                            self.current_iterator = macro_generated_reset();
                                                        } else {
                                                            self
                                                                .current_iterator = macro_generated_return_next(
                                                                self.current_iterator,
                                                            );
                                                        }
                                                        false
                                                    }
                                                }
                                            }
                                        }
                                    } {
                                        self.Velocity.index += 1;
                                    }
                                }
                                _ => {
                                    let result = Some((
                                        match &mut self.Position.vec {
                                            None => return None,
                                            Some(array) => {
                                                match array.get_mut(self.Position.index) {
                                                    None => return None,
                                                    Some(i_elem) => &mut i_elem.elem,
                                                }
                                            }
                                        },
                                        match &mut self.Velocity.vec {
                                            None => return None,
                                            Some(array) => {
                                                match array.get_mut(self.Velocity.index) {
                                                    None => return None,
                                                    Some(i_elem) => &mut i_elem.elem,
                                                }
                                            }
                                        },
                                    ));
                                    self.current_entity += 1;
                                    self.current_iterator = macro_generated_reset();
                                    return result;
                                }
                            }
                        }
                    }
                }
                MacroGeneratedComponentIterator::<Position, Velocity> {
                    current_iterator: macro_generated_reset(),
                    current_entity: 0,
                    Position: MacroGeneratedIterableVec {
                        vec: match comp_map.get::<ComponentArray<Position>>() {
                            None => None,
                            Some(comp_arr) => Some(comp_arr.get_array_mut()),
                        },
                        index: 0,
                    },
                    Velocity: MacroGeneratedIterableVec {
                        vec: match comp_map.get::<ComponentArray<Velocity>>() {
                            None => None,
                            Some(comp_arr) => Some(comp_arr.get_array_mut()),
                        },
                        index: 0,
                    },
                }
            }
        } { // iterate over mut causing the issue, while iterate over component works fine
            
            
            
            // doing stuff on pos and vel
            let some_var = pos.x + pos.y + vel.vx + vel.vy;
        }

    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    


}

