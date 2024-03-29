use foundry::*;
    // a position component



fn main() {
    
    // create world and entities
    let mut world = World::default();
    
    // create 10 entities with components
    let _entities = create_entities!(world; 10, |i| i, |i| format!("entity {}", i));
    let _entities2 = create_entities!(world; 10, |i| 10 + i);
    let _entities3 = create_entities!(world; 10, |i| format!("entity {}", 20 + i));

    // iterate over entities and print their components
    for (ent, name) in world.query1d::<String>() {
        println!("entity: {}, name: {}", ent, name);
    }
    // iterate over entities and print their components
    for (ent, index) in  world.query1d::<i32>() {
        println!("entity: {}, index: {}", ent, index);
    }
    // iterate over entities and print their components
    for (ent, index, name) in world.query2d::<i32, String>() {
        println!("entity: {}, index: {}, name: {}", ent, index, name);
    }


}