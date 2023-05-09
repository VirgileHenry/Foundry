use foundry::*;
    // a position component
struct Component1 {
    _value: u8,
}

struct Component2 {
    _value: f32,
}

struct Component3();

fn main() {
    
    // create world and entities
    let mut world = World::default();
    let entity = world.create_entity();

    // let's do stuff with components
    world.add_component(entity, Component1{_value:0});
    world.add_component(entity, Component2{_value:0.});

    let _ = get_components!(&world; entity; Component1, Component2, Component3);
    let _ = get_components!(&world; entity; Component1, Component2);


}