use foundry::*;
    // a position component
struct Component1 {
    value: u8,
}

struct Component2 {
    value: f32,
}

struct Component3();

fn main() {
    
    // create world and entities
    let mut world = World::new();
    let entity = world.create_entity();

    // let's do stuff with components
    world.add_component(entity, Component1{value:0});
    world.add_component(entity, Component2{value:0.});

    let components = &mut world.components;

    let s = get_components!(components; entity; mut Component1, Component2, Component3);
    let s2 = get_components!(components; entity; Component1, mut Component2);

}