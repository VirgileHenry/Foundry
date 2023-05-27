use crate as foundry; // simulate extern use ?
use foundry::*;




#[test]
/// Test the creation of entities, in all three ways.
fn single_component() {
    let mut world = World::default();

    // create an entity with a component
    let component = "Hello Component".to_string();
    let entity = world.create_entity();
    world.add_component(entity, component);

    let borrow = world.get_component_mut::<String>(entity).unwrap();
    *borrow = "Nothing is left".to_string();

    let component = world.remove_component::<String>(entity).unwrap();
    assert!(component == "Nothing is left".to_string());
}
