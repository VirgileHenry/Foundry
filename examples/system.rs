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

struct PhysicSystem {}

impl Updatable for PhysicSystem {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
        for (pos, vel) in component_iterator!(components; mut Position, Velocity) {
            pos.x += vel.vx * delta;
            pos.y += vel.vy * delta;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    
    use std::time::Instant;
    // create world and entities
    let mut world = World::default();
    let _entity = create_entities!(world; 1_000_000, |_:usize| { return Position{x:0.0, y:5.0}; }, |_:usize| { return Velocity{vx:0.0, vy:0.0}; });

    let physics = PhysicSystem {};

    let physic_system = System::new(Box::new(physics), UpdateFrequency::Fixed(0.002));

    world.register_system(physic_system, 1);

    let mut prev = Instant::now();

    for _ in 0..1000000 {
        let delta = prev.elapsed().as_secs_f64();

        world.update(delta as f32, &mut 0);
        
        prev = Instant::now();
    }

    [0, 1, 2].iter().peekable().peek();
}