use foundry::*;
    // a position component
struct Position {
    x: f32,
    y: f32,
}
// a velocity component
struct Velocity {
    vx: f32,
    vy: f32,
}

fn main() {
    
    use std::time::Instant;
    // create world and entities
    let mut world = World::new();
    let mut entities = create_entities!(world.components; 10, |i:usize| { return Position{x:i as f32, y: i as f32}; }, |i:usize| { return Velocity{vx:0.0, vy:0.0}; });
    for (pos, vel) in iterate_over_component!(world.components; Position, Velocity) {
        let a = pos.x + vel.vx;
    }

    // display all entities
    println!("Creating entities : ");
    for (ent, _pos) in iterate_over_component!(world.components; EntityRef; Position) {
        println!("entity : {}", ent.id);
    }

    // set half the entities as inactive !
    let mut iterator = entities.iter();
    while let Some(entity) = iterator.next() {
        iterator.next();
        world.set_entity_active(*entity, false);
    }

    // display all entities
    println!("setting half the entities to inactive : ");
    for (ent, _pos) in iterate_over_component!(world.components; EntityRef; Position) {
        println!("entity : {}", ent.id);
    }

    for entity in entities.iter() {
        world.set_entity_active(*entity, !world.is_entity_active(*entity).unwrap());
    }

    // display all entities
    println!("switching active entities : ");
    for (ent, _pos) in iterate_over_component!(world.components; EntityRef; Position) {
        println!("entity : {}", ent.id);
    }

    for entity in entities.iter() {
        world.set_entity_active(*entity, true);
    }

    // display all entities
    println!("All entities back to acitve : ");
    for (ent, _pos) in iterate_over_component!(world.components; EntityRef; Position) {
        println!("entity : {}", ent.id);
    }

}