use foundry::*;
    // a position component
struct Component1 {
    _value: u8,
}



fn main() {
    
    // create world and entities
    let mut world = World::default();
    let entity = world.create_entity();

    // let's do stuff with components
    world.add_component(entity, Component1{_value:0});

    // we can also add and get components that have more complicated structures
    world.add_component(entity, (1.0f32, "Hello, World!"));


}