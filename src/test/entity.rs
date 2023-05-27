use crate as foundry; // simulate extern use ?
use foundry::*;




#[test]
/// Test the creation of entities, in all three ways.
fn entity_creation() {
    let mut world = World::default();

    // create a new enmpty entity test
    let entity1 = world.create_entity();
    // create a vec of new entities
    let entities1 = world.create_entities(10);
    // create a new entity with components
    let entity2 = create_entity!(&mut world; 1, true);
    // create a bunch of entities with components
    let entities2 = create_entities!(world; 10, |i| i, |i| i % 2 == 0);

    // let's assert our entities are different and ordered.
    assert!(entity1 < entities1[0]);
    for ents in entities1.windows(2) {
        assert!(ents[0] < ents[1]);
    }
    assert!(entities1[9] < entity2);
    assert!(entity2 < entities2[0]);
    for ents in entities2.windows(2) {
        assert!(ents[0] < ents[1]);
    }
}