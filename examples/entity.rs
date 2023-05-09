use foundry::*;
    // a position component
struct Position {
    x: f32,
    y: f32,
}
// a velocity component
struct Velocity {
    vx: f32,
    vy: f32,
}



fn main() {
    
    // create world and entities
    let mut world = World::default();
    let mut _entity = create_entities!(world; 1_000_000, |_i:usize| { return Position{x:0.0, y:5.0}; }, |_i:usize| { return Velocity{vx:0.0, vy:0.0}; });

}