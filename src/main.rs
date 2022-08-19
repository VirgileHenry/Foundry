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
    let mut entities: Vec<Entity> = Vec::new();
    for i in 0..100 {
        entities.push(create_entity!(ecs; Position { x: i as f32, y: (100 - i) as f32 }, Velocity{vx: 0.0, vy:0.0}));
    }

    for component in ecs.components.iterate_over_1_component_mut::<Position>() {
        println!("reading positions : {} {}", component.x, component.y);
    }

    for component in ecs.components.iterate_over_1_component_mut::<Velocity>() {
        println!("reading velocity : {} {}", component.vx, component.vy);
    }

    for comps in ecs.components.iterate_over_2_component_mut::<Position, Velocity>() {
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
    
    for comps in ecs.components.iterate_over_2_component_mut::<Position, Velocity>() {
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
    
    for comps in ecs.components.iterate_over_2_component_mut::<Position, Velocity>() {
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
        for comps in components.iterate_over_2_component_mut::<Position, Velocity>() {
            let (pos, vel) = comps; // unpack
            println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
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

}

