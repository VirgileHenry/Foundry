use foundry::*;
    // a position component
struct Position {
    _x: f32,
    _y: f32,
}
// a velocity component
struct Velocity {
    _vx: f32,
    _vy: f32,
}



fn main() {
    
    // create world and entities
    let mut world = World::default();
    let mut _entity = create_entities!(world; 1_000_000, |_i:usize| { return Position{_x:0.0, _y:5.0}; }, |_i:usize| { return Velocity{_vx:0.0, _vy:0.0}; });

}