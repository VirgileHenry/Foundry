mod ecs;
use ecs::{
    entity::Entity,
    system::{
        Updatable, 
        System,
        UpdateFrequency
    }
};

use crate::ecs::ecs::World;
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
    let mut ecs: World = World::new();
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
    let mut ecs = World::new();
    // objective :
    // let entity = create_entity!(ecs, Position{...}, Velocity{...});
}

#[test]
fn iterate_component_test() {
    let mut ecs = World::new();
    let mut entities: Vec<Entity> = create_entities!(ecs; 100, |i:usize| -> Position {Position{x:i as f32, y:i as f32}} );

    for component in iterate_over_component!(&ecs.components; Position) {
        println!("reading positions : {} {}", component.x, component.y);
    }

    for component in iterate_over_component!(&ecs.components; Velocity) {
        println!("reading velocity : {} {}", component.vx, component.vy);
    }

    for component in iterate_over_component!(&ecs.components; Velocity) {
        println!("writing velocity : {} {}", component.vx, component.vy);
    }

    for comps in iterate_over_component!(&ecs.components; Position, Velocity) {
        let (pos, vel) = comps; // unpack
        println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
    }

    ecs = World::new();
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
    
    for comps in iterate_over_component!(&ecs.components; Position, Velocity) {
        let (pos, vel) = comps; // unpack
        println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
    }

    ecs = World::new();
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
    
    for comps in iterate_over_component!(&ecs.components; Position, Velocity) {
        let (pos, vel) = comps; // unpack
        println!("Found two components on entity : pos({} {}) and vel({} {})", pos.x, pos.y, vel.vx, vel.vy);
    }

    println!("Iterate in an iteration test : ");

    for pos in iterate_over_component!(&ecs.components; Position) {
        for vel in iterate_over_component!(&ecs.components; Velocity) {
            // not mut yet, when will we do this ?
        }
    }

}


struct PhysicSystem {
    gravity_x: f32,
    gravity_y: f32,
}


impl Updatable for PhysicSystem {
    fn update(&mut self, components: &mut ecs::component_table::ComponentTable, delta: f32) {
        let mut i = 0;
        for (pos, vel) in iterate_over_component_mut!(components; Position, Velocity) {
            vel.vx += self.gravity_x * delta;
            vel.vy += self.gravity_y * delta;
            pos.x += vel.vx * delta;
            pos.y += vel.vy * delta;

            // simple collision
            if pos.y < 0.0 {
                pos.y = -pos.y;
                vel.vy = -0.8 * vel.vy;
            }
            i += 1;

            // println!("pos : {} {}     vel : {} {}", pos.x, pos.y, vel.vx, vel.vy);
        }
        // println!("{}", delta);
    }
}


#[test]
fn system_test() {
    
    use std::time::Instant;

    // Code block to measure.
    // create ecs and entities
    let mut ecs = World::new();
    let mut entity = create_entities!(ecs; 1_000_000, |i:usize| { return Position{x:0.0, y:5.0}; }, |i:usize| { return Velocity{vx:0.0, vy:0.0}; });

    let physics = PhysicSystem {
        gravity_x: 0.0,
        gravity_y: -9.81,
    };

    let physic_system = System::new(Box::new(physics), UpdateFrequency::Fixed(0.002));

    ecs.register_system(physic_system, 1);

    let mut prev = Instant::now();

    loop {
        let mut delta = prev.elapsed().as_secs_f64();

        delta = 0.0001;

        println!("{}", delta);

        ecs.update(delta as f32);
        let pos = ecs.get_component::<Position>(&entity[0]).unwrap();
        
        println!("pos : {} {}", pos.x, pos.y);

        prev = Instant::now();
    }

}

#[test]
fn entity_active_test() {
    let mut world = World::new();
    let entity = world.create_entity();
    world.add_component::<Position>(&entity, Position{x:0.0, y:0.0});
    let entity1 = world.create_entity();
    world.add_component::<Position>(&entity1, Position{x:0.0, y:0.0});
    world.set_entity_active(&entity, false);
    for pos in iterate_over_component!(world.components; Position) {
        println!("found pos !"); // should happen once, because entity is deactivated
    }
}