use foundry::*;
// a position component
#[allow(unused)]
struct Position {
    x: f32,
    y: f32,
}
// a velocity component
#[allow(unused)]
struct Velocity {
    vx: f32,
    vy: f32,
}

fn main() {
    
    // create world and entities
    let mut world = World::default();
    let entities = create_entities!(world; 10, |i:usize| { return Position{x:i as f32, y: i as f32}; }, |_| { return Velocity{vx:0.0, vy:0.0}; });
    for (pos, vel) in iterate_over_component!(world; Position, Velocity) {
        let _a = pos.x + vel.vx;
    }

    // display all entities
    println!("Creating entities : ");
    for (ent, _pos) in iterate_over_component!(world; EntityRef; Position) {
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
    for (ent, _pos) in iterate_over_component!(world; EntityRef; Position) {
        println!("entity : {}", ent.id);
    }

    for entity in entities.iter() {
        let active = world.is_entity_active(*entity).unwrap();
        world.set_entity_active(*entity, !active);
    }

    // display all entities
    println!("switching active entities : ");
    for (ent, _pos) in iterate_over_component!(world; EntityRef; Position) {
        println!("entity : {}", ent.id);
    }

    for entity in entities.iter() {
        world.set_entity_active(*entity, true);
    }

    // display all entities
    println!("All entities back to acitve : ");
    for (ent, _pos) in iterate_over_component!(world; EntityRef; Position) {
        println!("entity : {}", ent.id);
    }

}