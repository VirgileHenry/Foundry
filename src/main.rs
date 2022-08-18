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
    let entity1: Entity = ecs.create_entity_with_1(Position{x:1.0, y:0.5});
    let entity2: Entity = ecs.create_entity_with_2(Position{x: 0.2, y: 1.3}, Velocity{vx:0.1, vy:-0.3});
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
        entities.push(ecs.create_entity());
        ecs.add_component::<Position>(entities.get(i).unwrap(), Position { x: i as f32, y: (100 - i) as f32 });
    }

    for component in ecs.components.iterate_over_1_component_mut::<Position>() {
        println!("reading positions : {} {}", component.x, component.y);
    }
}


struct PhysicSystem {
    gravity_x: f32,
    gravity_y: f32,
}

impl Updatable for PhysicSystem {
    fn update(&mut self, components: &ecs::component_table::ComponentTable, delta: f32) {
        /*
        match components.iterate_over_components_2::<Position, Velocity>() {

        }
        */
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

