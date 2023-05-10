use foundry::{World, create_entities, component_iterator};



fn main() {
    // test out the layers of the foundry ! 
    // insert 6 entities
    let mut world = World::default();

    let entities = create_entities!(world; 6,
        |i| format!("Entity {i}") 
    );

    // set the first 2 entities on NOT layer, 2
    for i in 0..2 {
        world.set_entity_layer(entities[i], 2, false);
    }
    // then, set the next two to be only at 2
    for i in 2..4 {
        world.set_entity_layers(entities[i], 1 << 2);
    }
    // the last two will have full mask

    // now, let's iterate with masks : 
    println!("Expecting all 6 entities"); 
    for ent in component_iterator!(&world; String) {
        println!("{ent}");
    }

    println!("Expecting all 2->5 entities");
    // now, let's iterate with only the second layer masks : 
    for ent in component_iterator!(&world, 1 << 2; String) {
        println!("{ent}");
    }

    println!("Expecting 0,1,4,5 entities");
    // finally, on everything but the second layer :
    for ent in component_iterator!(&world, !(1 << 2); String) {
        println!("{ent}");
    }
}