use crate as foundry; // simulate extern use ? for macros mostly.
use foundry::*;




#[test]
/// Test the creation of entities, in all three ways.
fn comp_iter() {
    let mut world = World::default();

    create_entities!(world; 100, |i| i, |i| format!("{i}"));

    // try different combinations of iterations
    for _ in component_iterator!(&world; usize) {}
    for _ in component_iterator!(&world; String) {}
    for _ in component_iterator!(&world; mut usize) {}
    for _ in component_iterator!(&world; mut String) {}
    // for _ in component_iterator!(&world; usize, String) {}
    for _ in component_iterator!(&world; mut usize, String) {}
    // for _ in component_iterator!(&world; usize, mut String) {}
    for _ in component_iterator!(&world; mut usize, mut String) {}
    // if all of these pass, it should get pretty expandable from here.
}

