use foundry::*;
    // a position component
struct Component1 {
    value: u8,
}



fn main() {
    
    use std::time::Instant;
    // create world and entities
    let mut world = World::new();
    let entity = world.create_entity();

    // let's do stuff with components
    world.add_component(entity, Component1{value:0});


}