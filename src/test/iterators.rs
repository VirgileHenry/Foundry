use crate as foundry; // simulate extern use ? for macros mostly.
use foundry::*;




#[test]
/// Test the creation of entities, in all three ways.
fn comp_iter() {
    let mut world = World::default();

    create_entities!(world; 5, |i| i as usize, |i| format!("{i}"));

    let mut iter = world.query1d::<usize>();
    assert_eq!(iter.next(), Some((0, &0)));
    assert_eq!(iter.next(), Some((1, &1)));
    assert_eq!(iter.next(), Some((2, &2)));
    assert_eq!(iter.next(), Some((3, &3)));
    assert_eq!(iter.next(), Some((4, &4)));
    assert_eq!(iter.next(), None);

    let mut iter = world.query1d::<String>();
    assert_eq!(iter.next(), Some((0, &"0".to_string())));
    assert_eq!(iter.next(), Some((1, &"1".to_string())));
    assert_eq!(iter.next(), Some((2, &"2".to_string())));
    assert_eq!(iter.next(), Some((3, &"3".to_string())));
    assert_eq!(iter.next(), Some((4, &"4".to_string())));
    assert_eq!(iter.next(), None);

    let mut iter = world.query2d::<usize, String>();
    assert_eq!(iter.next(), Some((0, &0, &"0".to_string())));
    assert_eq!(iter.next(), Some((1, &1, &"1".to_string())));
    assert_eq!(iter.next(), Some((2, &2, &"2".to_string())));
    assert_eq!(iter.next(), Some((3, &3, &"3".to_string())));
    assert_eq!(iter.next(), Some((4, &4, &"4".to_string())));
    assert_eq!(iter.next(), None);

    let mut iter = world.query1d_mut::<usize>();
    assert_eq!(iter.next(), Some((0, &mut 0)));
    assert_eq!(iter.next(), Some((1, &mut 1)));
    assert_eq!(iter.next(), Some((2, &mut 2)));
    assert_eq!(iter.next(), Some((3, &mut 3)));
    assert_eq!(iter.next(), Some((4, &mut 4)));
    assert_eq!(iter.next(), None);

    let mut iter = world.query1d_mut::<String>();
    assert_eq!(iter.next(), Some((0, &mut "0".to_string())));
    assert_eq!(iter.next(), Some((1, &mut "1".to_string())));
    assert_eq!(iter.next(), Some((2, &mut "2".to_string())));
    assert_eq!(iter.next(), Some((3, &mut "3".to_string())));
    assert_eq!(iter.next(), Some((4, &mut "4".to_string())));
    assert_eq!(iter.next(), None);

    let mut iter = world.query2d_mut::<usize, String>();
    assert_eq!(iter.next(), Some((0, &mut 0, &mut "0".to_string())));
    assert_eq!(iter.next(), Some((1, &mut 1, &mut "1".to_string())));
    assert_eq!(iter.next(), Some((2, &mut 2, &mut "2".to_string())));
    assert_eq!(iter.next(), Some((3, &mut 3, &mut "3".to_string())));
    assert_eq!(iter.next(), Some((4, &mut 4, &mut "4".to_string())));
    assert_eq!(iter.next(), None);
    

}

